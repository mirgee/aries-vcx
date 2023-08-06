use std::sync::Arc;

use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{extra_fields::KeyKind, service::ServiceSov, DidDocumentSov};
use did_parser::{Did, DidUrl};
use did_peer::peer_did::generate::generate_numalgo2;
use did_resolver_registry::ResolverRegistry;
use messages::{
    decorators::thread::Thread,
    msg_fields::protocols::did_exchange::{
        complete::Complete,
        request::Request,
        response::{Response, ResponseContent, ResponseDecorators},
    },
};
use public_key::{Key, KeyType};

use crate::{
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    protocols::{
        did_exchange::{
            helpers::{attach_to_ddo_sov, ddo_sov_to_attach},
            initiation_type::Responder,
            record::ConnectionRecord,
            service::{did_doc_from_keys, generate_keypair},
            states::{completed::Completed, responder::response_sent::ResponseSent},
            transition::{transition_error::TransitionError, transition_result::TransitionResult},
        },
        mediated_connection::pairwise_info::PairwiseInfo,
    },
};

use super::DidExchangeService;

pub type DidExchangeServiceResponder<S> = DidExchangeService<Responder, S>;

async fn create_our_did_document(
    wallet: &Arc<dyn BaseWallet>,
    service: ServiceSov,
) -> Result<(DidDocumentSov, Did, Key), AriesVcxError> {
    let key_ver = generate_keypair(wallet, KeyType::Ed25519).await?;
    let key_enc = generate_keypair(wallet, KeyType::X25519).await?;

    let did_document_temp = did_doc_from_keys(Default::default(), key_ver.clone(), key_enc.clone(), service.clone());
    let peer_did = generate_numalgo2(did_document_temp.into())?;

    Ok((
        did_doc_from_keys(peer_did.clone().into(), key_ver, key_enc.clone(), service),
        peer_did.into(),
        key_enc,
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
        let (our_ddo, peer_did, enc_key) = create_our_did_document(wallet, service.clone()).await?;

        if request.decorators.thread.and_then(|t| t.pthid) != Some(invitation_id.clone()) {
            return Err(AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidState,
                "Parent thread ID of the request does not match the id of the invite",
            ));
        }
        // TODO The DDO must be signed by the pw vk in the recipient keys of the invitation
        // (probably use a new trait for this)

        let content = ResponseContent {
            did: peer_did.to_string(),
            did_doc: Some(ddo_sov_to_attach(our_ddo.clone())),
        };
        let thread = {
            let mut thread = Thread::new(request.id.clone());
            thread.pthid = Some(invitation_id.clone());
            thread
        };
        let decorators = ResponseDecorators { thread, timing: None };
        let response = Response::with_decorators(request.id.clone(), content, decorators);
        Ok(TransitionResult {
            state: DidExchangeService::from_parts(
                ResponseSent {
                    request_id: request.id,
                    invitation_id,
                },
                Responder,
                their_ddo,
                enc_key,
            ),
            output: response,
        })
    }
}

impl DidExchangeServiceResponder<ResponseSent> {
    pub fn receive_complete(self, complete: Complete) -> Result<DidExchangeServiceResponder<Completed>, AriesVcxError> {
        if complete.decorators.thread.thid != self.state.request_id {
            todo!()
        }
        if complete.decorators.thread.pthid != Some(self.state.invitation_id.to_string()) {
            todo!()
        }
        Ok(DidExchangeService::from_parts(
            Completed,
            Responder,
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
