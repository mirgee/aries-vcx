use std::sync::Arc;

use aries_vcx_core::{ledger::base_ledger::IndyLedgerRead, wallet::base_wallet::BaseWallet};
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{
    extra_fields::{didcommv1::ExtraFieldsDidCommV1, didcommv2::ExtraFieldsDidCommV2, KeyKind},
    service::{aip1::ServiceAIP1, didcommv1::ServiceDidCommV1, didcommv2::ServiceDidCommV2, ServiceSov},
    DidDocumentSov,
};
use did_key::DidKey;
use did_parser::Did;
use did_peer::{peer_did::generate::generate_numalgo2, peer_did_resolver::resolver::PeerDidResolver};
use did_resolver::traits::resolvable::DidResolvable;
use messages::{
    decorators::thread::ThreadGoalCode,
    msg_fields::protocols::{
        did_exchange::{complete::Complete, request::Request, response::Response},
        out_of_band::{
            invitation::{Invitation as OobInvitation, OobService},
            OobGoalCode,
        },
    },
};
use public_key::{Key, KeyType};
use shared_vcx::maybe_known::MaybeKnown;
use url::Url;

use crate::{
    common::{
        keys::get_verkey_from_ledger,
        ledger::transactions::{into_did_doc, into_did_doc_1, resolve_service},
    },
    errors::error::AriesVcxError,
    handlers::util::AnyInvitation,
    protocols::{
        did_exchange::{
            helpers::attach_to_ddo_sov,
            initiation_type::Requester,
            protocol::requester::{DidExchangeRequestParams, DidExchangeRequester},
            record::ConnectionRecord,
            states::{completed::Completed, requester::request_sent::RequestSent},
            transition::transition_result::TransitionResult,
        },
        mediated_connection::pairwise_info::PairwiseInfo,
    },
    utils::{from_legacy_did_doc_to_sov, from_legacy_service_to_service_sov},
};

use super::DidExchangeService;

pub type DidExchangeServiceRequester<S> = DidExchangeService<Requester, S>;

fn map_goal_code(oob_goal_code: MaybeKnown<OobGoalCode>) -> MaybeKnown<ThreadGoalCode> {
    match oob_goal_code {
        MaybeKnown::Known(goal_code) => match goal_code {
            OobGoalCode::IssueVC => MaybeKnown::Known(ThreadGoalCode::AriesVcIssue),
            OobGoalCode::RequestProof => MaybeKnown::Known(ThreadGoalCode::AriesVcVerify),
            OobGoalCode::CreateAccount | OobGoalCode::P2PMessaging => MaybeKnown::Known(ThreadGoalCode::AriesRelBuild),
        },
        MaybeKnown::Unknown(goal_code) => MaybeKnown::Unknown(goal_code),
    }
}

pub struct PairwiseConstructRequestConfig {
    pub invitation: OobInvitation,
    pub wallet: Arc<dyn BaseWallet>,
    pub service_endpoint: Url,
    pub routing_keys: Vec<String>,
}

pub struct PublicConstructRequestConfig {
    pub their_did: Did,
    pub our_did: Did,
}

pub enum ConstructRequestConfig {
    Pairwise(PairwiseConstructRequestConfig),
    Public(PublicConstructRequestConfig),
}

impl DidExchangeServiceRequester<RequestSent> {
    // TODO: The invitation must contain didexchage handshake protocol
    #[allow(dead_code, unused)]
    async fn construct_request_pairwise(
        ledger: Arc<dyn IndyLedgerRead>,
        PairwiseConstructRequestConfig {
            wallet,
            service_endpoint,
            routing_keys,
            invitation,
        }: PairwiseConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        // TODO: We don't need the whole PairwiseInfo, just the verkey
        let pairwise_info = PairwiseInfo::create(&wallet).await?;
        let our_did_document = {
            let our_temp_did: Did = format!("did:sov:{}", pairwise_info.pw_did).parse()?;
            // We must send DIDCommV1
            // This is retarded. We must create two ver. methods using the same key, then take the
            // key bytes to generate Key, then use IT to create a DidKey to set as recipient key in
            // the service to use in the DDO which is used to create the peer did to send in the
            // request.
            let vm_ver = VerificationMethod::builder(
                our_temp_did.clone().into(),
                our_temp_did.clone(),
                VerificationMethodType::Ed25519VerificationKey2020,
            )
            .add_public_key_base58(pairwise_info.pw_vk.clone())
            .build();
            let vm_ka = VerificationMethod::builder(
                our_temp_did.clone().into(),
                our_temp_did.clone(),
                VerificationMethodType::X25519KeyAgreementKey2020,
            )
            .add_public_key_base58(pairwise_info.pw_vk.clone())
            .build();
            let key = Key::new(vm_ver.public_key().key_decoded().unwrap(), KeyType::X25519).unwrap();
            let extra = ExtraFieldsDidCommV1::builder()
                .set_routing_keys(routing_keys.into_iter().map(KeyKind::Value).collect())
                .set_recipient_keys(vec![KeyKind::DidKey(key.try_into().unwrap())])
                .build();
            let service = ServiceSov::DIDCommV1(ServiceDidCommV1::new(
                Default::default(),
                service_endpoint.into(),
                extra,
            )?);
            DidDocumentSov::builder(our_temp_did)
                .add_service(service)
                .add_verification_method(vm_ver.clone())
                .add_key_agreement(vm_ka)
                .build()
        };
        let their_did_document = into_did_doc_1(&AnyInvitation::Oob(invitation.clone())).await?;
        let our_peer_did = generate_numalgo2(our_did_document.clone().into())?;
        let params = DidExchangeRequestParams {
            invitation_id: invitation.id.clone(),
            label: invitation.content.label.unwrap_or_default().clone(),
            // Must be non-empty for some reason
            goal: Some("To establish a connection".to_string()),
            goal_code: Some(MaybeKnown::Known(ThreadGoalCode::AriesRelBuild)),
            did: our_peer_did.clone().into(),
            did_doc: Some(our_did_document),
        };
        let TransitionResult {
            state: sm,
            output: request,
        } = DidExchangeRequester::construct_request(params)?;
        Ok(TransitionResult {
            state: Self {
                sm,
                pairwise_info: PairwiseInfo {
                    pw_did: our_peer_did.to_string(),
                    pw_vk: pairwise_info.pw_vk,
                },
                did_document: their_did_document,
            },
            output: request,
        })
    }

    async fn construct_request_public(
        ledger: Arc<dyn IndyLedgerRead>,
        PublicConstructRequestConfig { their_did, our_did }: PublicConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        let service = resolve_service(&ledger, &OobService::Did(their_did.id().to_string())).await?;
        // TODO: If it's on the ledger but in the wallet, we may not know it but we have a problem
        let our_verkey = get_verkey_from_ledger(&ledger, &our_did.id().to_string()).await?;
        let vm = VerificationMethod::builder(
            their_did.clone().into(),
            their_did.clone(),
            VerificationMethodType::Ed25519VerificationKey2018,
        )
        .add_public_key_base58(service.recipient_keys.first().unwrap().clone())
        .build();
        let their_did_document = DidDocumentSov::builder(their_did.clone())
            .add_service(from_legacy_service_to_service_sov(service.clone())?)
            .add_controller(their_did.clone())
            .add_verification_method(vm)
            .build();
        let invitation_id = format!("{}#{}", their_did, service.id);
        let params = DidExchangeRequestParams {
            invitation_id,
            label: "".to_string(),
            goal: Some("To establish a connection".to_string()),
            goal_code: Some(MaybeKnown::Known(ThreadGoalCode::AriesRelBuild)),
            did: our_did.clone(),
            did_doc: None,
        };
        let TransitionResult {
            state: sm,
            output: request,
        } = DidExchangeRequester::construct_request(params)?;
        Ok(TransitionResult {
            state: Self {
                sm,
                pairwise_info: PairwiseInfo {
                    pw_did: their_did.to_string(),
                    pw_vk: our_verkey,
                },
                did_document: their_did_document,
            },
            output: request,
        })
    }

    pub async fn construct_request(
        ledger: Arc<dyn IndyLedgerRead>,
        config: ConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        match config {
            ConstructRequestConfig::Pairwise(config) => Self::construct_request_pairwise(ledger, config).await,
            ConstructRequestConfig::Public(config) => Self::construct_request_public(ledger, config).await,
        }
    }

    pub async fn receive_response(
        self,
        response: Response,
    ) -> Result<TransitionResult<DidExchangeServiceRequester<Completed>, Complete>, AriesVcxError> {
        let TransitionResult {
            state: sm,
            output: complete,
        } = self.sm.construct_complete(response.clone())?;
        let did_document = if let Some(ddo) = response.content.did_doc {
            attach_to_ddo_sov(ddo)?
        } else {
            PeerDidResolver::new()
                .resolve(&response.content.did.parse()?, &Default::default())
                .await?
                .did_document()
                .to_owned()
                .into()
        };
        Ok(TransitionResult {
            state: DidExchangeServiceRequester {
                sm,
                pairwise_info: self.pairwise_info,
                did_document,
            },
            output: complete,
        })
    }
}

impl DidExchangeServiceRequester<Completed> {
    pub fn to_record(self) -> ConnectionRecord {
        ConnectionRecord::from_parts(self.did_document, self.pairwise_info)
    }
}
