use std::sync::Arc;

use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{extra_fields::KeyKind, service::ServiceSov, DidDocumentSov};
use did_parser::{Did, DidUrl};
use did_peer::peer_did::generate::generate_numalgo2;
use did_resolver_registry::ResolverRegistry;
use messages::msg_fields::protocols::did_exchange::{complete::Complete, request::Request, response::Response};
use public_key::KeyType;

use crate::{
    errors::error::AriesVcxError,
    protocols::{
        did_exchange::{
            helpers::attach_to_ddo_sov,
            initiation_type::Responder,
            protocol::responder::{DidExchangeResponder, DidExchangeResponseParams},
            record::ConnectionRecord,
            service::{did_doc_from_keys, generate_keypair},
            states::{completed::Completed, responder::response_sent::ResponseSent},
            transition::transition_result::TransitionResult,
        },
        mediated_connection::pairwise_info::PairwiseInfo,
    },
};

use super::DidExchangeService;

pub type DidExchangeServiceResponder<S> = DidExchangeService<Responder, S>;

async fn create_our_did_document(
    wallet: &Arc<dyn BaseWallet>,
    service: ServiceSov,
) -> Result<(DidDocumentSov, Did), AriesVcxError> {
    let key_ver = generate_keypair(wallet, KeyType::Ed25519).await?;
    let key_enc = generate_keypair(wallet, KeyType::X25519).await?;

    let did_document_temp = did_doc_from_keys(Default::default(), key_ver.clone(), key_enc.clone(), service.clone());
    let peer_did = generate_numalgo2(did_document_temp.into())?;

    Ok((
        did_doc_from_keys(peer_did.clone().into(), key_ver, key_enc, service),
        peer_did.into(),
    ))
}

async fn resolve_their_ddo(
    resolver_registry: &Arc<ResolverRegistry>,
    request: &Request,
) -> Result<DidDocumentSov, AriesVcxError> {
    if let Some(ddo) = request.content.did_doc.clone().map(attach_to_ddo_sov).transpose()? {
        Ok(ddo)
    } else {
        Ok(resolver_registry
            .resolve(&request.content.did.parse()?, &Default::default())
            .await?
            .did_document()
            .to_owned()
            .into())
    }
}

impl DidExchangeServiceResponder<ResponseSent> {
    pub async fn receive_request(
        wallet: &Arc<dyn BaseWallet>,
        resolver_registry: &Arc<ResolverRegistry>,
        request: Request,
        // TODO: We need just the service endpoint and routing keys
        service: ServiceSov,
        invitation_id: String,
    ) -> Result<TransitionResult<DidExchangeServiceResponder<ResponseSent>, Response>, AriesVcxError> {
        let their_ddo = resolve_their_ddo(resolver_registry, &request).await?;
        let (our_ddo, peer_did) = create_our_did_document(wallet, service.clone()).await?;

        let params = DidExchangeResponseParams {
            request,
            did: peer_did.clone().into(),
            did_doc: Some(our_ddo),
            invitation_id,
        };
        let TransitionResult { state, output } = DidExchangeResponder::<ResponseSent>::construct_response(params)?;
        Ok(TransitionResult {
            state: DidExchangeService::from_parts(
                state,
                their_ddo,
                PairwiseInfo {
                    pw_did: peer_did.to_string(),
                    pw_vk: PairwiseInfo::create(wallet).await?.pw_vk,
                },
            ),
            output,
        })
    }
}

impl DidExchangeServiceResponder<ResponseSent> {
    pub fn receive_complete(self, complete: Complete) -> Result<DidExchangeServiceResponder<Completed>, AriesVcxError> {
        let state = self.sm.receive_complete(complete)?;
        Ok(DidExchangeService::from_parts(
            state,
            self.their_did_document,
            self.our_verkey,
        ))
    }
}

impl DidExchangeServiceResponder<Completed> {
    pub fn to_record(self) -> ConnectionRecord {
        ConnectionRecord::from_parts(self.their_did_document, self.our_verkey)
    }
}
