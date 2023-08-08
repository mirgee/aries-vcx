use std::sync::Arc;

use aries_vcx::{
    agency_client::{agency_client::AgencyClient, configuration::AgentProvisionConfig},
    common::ledger::{
        service_didsov::{DidSovServiceType, EndpointDidSov},
        transactions::{add_new_did, write_endpoint},
    },
    core::profile::{profile::Profile, vdrtools_profile::VdrtoolsProfile},
    global::settings::init_issuer_config,
    utils::provision::provision_cloud_agent,
};
use aries_vcx_core::indy::{
    ledger::pool::{create_pool_ledger_config, indy_open_pool, PoolConfigBuilder},
    wallet::{create_wallet_with_master_secret, open_wallet, wallet_configure_issuer, WalletConfig},
};
use did_peer::peer_did_resolver::resolver::PeerDidResolver;
use did_resolver_registry::ResolverRegistry;
use did_resolver_sov::{reader::ConcreteAttrReader, resolution::DidSovResolver};
use url::Url;

use crate::{
    agent::{agent_config::AgentConfig, agent_struct::Agent},
    error::AgentResult,
    services::{
        connection::{ServiceConnections, ServiceEndpoint},
        credential_definition::ServiceCredentialDefinitions,
        did_exchange::ServiceDidExchange,
        holder::ServiceCredentialsHolder,
        issuer::ServiceCredentialsIssuer,
        mediated_connection::ServiceMediatedConnections,
        out_of_band::ServiceOutOfBand,
        prover::ServiceProver,
        revocation_registry::ServiceRevocationRegistries,
        schema::ServiceSchemas,
        verifier::ServiceVerifier,
    },
};

pub struct AgencyInitConfig {
    pub agency_endpoint: Url,
    pub agency_did: String,
    pub agency_verkey: String,
}

pub struct WalletInitConfig {
    pub wallet_name: String,
    pub wallet_key: String,
    pub wallet_kdf: String,
}

pub struct PoolInitConfig {
    pub genesis_path: String,
    pub pool_name: String,
}

pub struct InitConfig {
    pub enterprise_seed: String,
    pub pool_config: PoolInitConfig,
    pub agency_config: Option<AgencyInitConfig>,
    pub wallet_config: WalletInitConfig,
    pub service_endpoint: ServiceEndpoint,
}

impl Agent {
    pub async fn initialize(init_config: InitConfig) -> AgentResult<Self> {
        let config_wallet = WalletConfig {
            wallet_name: init_config.wallet_config.wallet_name,
            wallet_key: init_config.wallet_config.wallet_key,
            wallet_key_derivation: init_config.wallet_config.wallet_kdf,
            wallet_type: None,
            storage_config: None,
            storage_credentials: None,
            rekey: None,
            rekey_derivation_method: None,
        };

        create_wallet_with_master_secret(&config_wallet).await.unwrap();
        let wallet_handle = open_wallet(&config_wallet).await.unwrap();

        let config_issuer = wallet_configure_issuer(wallet_handle, &init_config.enterprise_seed)
            .await
            .unwrap();
        init_issuer_config(&config_issuer.institution_did).unwrap();

        let pool_config = PoolConfigBuilder::default()
            .genesis_path(&init_config.pool_config.genesis_path)
            .build()
            .expect("Failed to build pool config");
        create_pool_ledger_config(
            &init_config.pool_config.pool_name,
            &init_config.pool_config.genesis_path,
        )
        .unwrap();
        let pool_handle = indy_open_pool(&init_config.pool_config.pool_name, pool_config.pool_config)
            .await
            .unwrap();

        let indy_profile = VdrtoolsProfile::init(wallet_handle, pool_handle);
        let profile: Arc<dyn Profile> = Arc::new(indy_profile);
        let wallet = profile.inject_wallet();

        // TODO: This setup should be easier
        // The default issuer did can't be used - its verkey is not in base58 - TODO: double-check
        let (public_did, _verkey) = add_new_did(
            &wallet,
            &profile.inject_indy_ledger_write(),
            &config_issuer.institution_did,
            None,
        )
        .await?;
        let endpoint = EndpointDidSov::create()
            .set_service_endpoint(init_config.service_endpoint.clone())
            .set_types(Some(vec![DidSovServiceType::Endpoint]));
        write_endpoint(&profile.inject_indy_ledger_write(), &public_did, &endpoint).await?;

        let did_peer_resolver = PeerDidResolver::new();
        let did_sov_resolver =
            DidSovResolver::new(Arc::<ConcreteAttrReader>::new(profile.inject_indy_ledger_read().into()));
        let did_resolver_registry = Arc::new(
            ResolverRegistry::new()
                .register_resolver::<PeerDidResolver>("peer".into(), did_peer_resolver.into())
                .register_resolver::<DidSovResolver>("sov".into(), did_sov_resolver.into()),
        );

        let (mediated_connections, config_agency_client) = if let Some(agency_config) = init_config.agency_config {
            let config_provision_agent = AgentProvisionConfig {
                agency_did: agency_config.agency_did,
                agency_verkey: agency_config.agency_verkey,
                agency_endpoint: agency_config.agency_endpoint,
                agent_seed: None,
            };
            let mut agency_client = AgencyClient::new();
            let config_agency_client = provision_cloud_agent(&mut agency_client, wallet, &config_provision_agent)
                .await
                .unwrap();
            (
                Some(Arc::new(ServiceMediatedConnections::new(
                    Arc::clone(&profile),
                    config_agency_client.clone(),
                ))),
                Some(config_agency_client),
            )
        } else {
            (None, None)
        };

        let connections = Arc::new(ServiceConnections::new(
            Arc::clone(&profile),
            init_config.service_endpoint.clone(),
        ));
        let did_exchange = Arc::new(ServiceDidExchange::new(
            Arc::clone(&profile),
            did_resolver_registry.clone(),
            init_config.service_endpoint.clone(),
            public_did,
        ));
        let out_of_band = Arc::new(ServiceOutOfBand::new(
            Arc::clone(&profile),
            init_config.service_endpoint,
        ));
        let schemas = Arc::new(ServiceSchemas::new(
            Arc::clone(&profile),
            config_issuer.institution_did.clone(),
        ));
        let cred_defs = Arc::new(ServiceCredentialDefinitions::new(Arc::clone(&profile)));
        let rev_regs = Arc::new(ServiceRevocationRegistries::new(
            Arc::clone(&profile),
            config_issuer.institution_did.clone(),
        ));
        let issuer = Arc::new(ServiceCredentialsIssuer::new(Arc::clone(&profile), connections.clone()));
        let holder = Arc::new(ServiceCredentialsHolder::new(Arc::clone(&profile), connections.clone()));
        let verifier = Arc::new(ServiceVerifier::new(Arc::clone(&profile), connections.clone()));
        let prover = Arc::new(ServiceProver::new(Arc::clone(&profile), connections.clone()));

        Ok(Self {
            profile,
            connections,
            did_exchange,
            mediated_connections,
            out_of_band,
            schemas,
            cred_defs,
            rev_regs,
            issuer,
            holder,
            verifier,
            prover,
            config: AgentConfig {
                config_wallet,
                config_issuer,
                config_agency_client,
            },
        })
    }
}
