use std::sync::Arc;

use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{service::ServiceSov, DidDocumentSov};
use did_parser::{Did, DidUrl};
use did_peer::peer_did::generate::generate_numalgo2;
use did_resolver_registry::ResolverRegistry;
use messages::msg_fields::protocols::did_exchange::{complete::Complete, request::Request, response::Response};

use crate::{
    errors::error::AriesVcxError,
    protocols::{
        did_exchange::{
            helpers::attach_to_ddo_sov,
            initiation_type::Responder,
            protocol::responder::{DidExchangeResponder, DidExchangeResponseParams},
            record::ConnectionRecord,
            states::{completed::Completed, responder::response_sent::ResponseSent},
            transition::transition_result::TransitionResult,
        },
        mediated_connection::pairwise_info::PairwiseInfo,
    },
};

use super::DidExchangeService;

pub type DidExchangeServiceResponder<S> = DidExchangeService<Responder, S>;

impl DidExchangeServiceResponder<ResponseSent> {
    pub async fn receive_request(
        wallet: &Arc<dyn BaseWallet>,
        resolver_registry: &Arc<ResolverRegistry>,
        request: Request,
        service: ServiceSov,
        invitation_id: String,
    ) -> Result<TransitionResult<DidExchangeServiceResponder<ResponseSent>, Response>, AriesVcxError> {
        let pairwise_info = PairwiseInfo::create(wallet).await?;
        let their_ddo = if let Some(ddo) = request.content.did_doc.clone().map(attach_to_ddo_sov).transpose()? {
            ddo
        } else {
            resolver_registry
                .resolve(&request.content.did.parse()?, &Default::default())
                .await?
                .did_document()
                .to_owned()
                .into()
        };

        // This is just to make the parsing work
        let did: Did = format!("did:sov:{}", pairwise_info.pw_did).parse()?;
        let did_url: DidUrl = did.clone().into();
        let our_ddo = {
            let vm =
                VerificationMethod::builder(did_url, did.clone(), VerificationMethodType::Ed25519VerificationKey2018)
                    .add_public_key_base58(pairwise_info.pw_vk.clone())
                    .build();
            DidDocumentSov::builder(did)
                .add_service(service.clone())
                .add_verification_method(vm)
                .build()
        };
        let peer_did = generate_numalgo2(our_ddo.into())?;

        let params = DidExchangeResponseParams {
            request,
            did: peer_did.clone().into(),
            did_doc: None,
            invitation_id,
        };
        let TransitionResult { state, output } = DidExchangeResponder::<ResponseSent>::construct_response(params)?;
        Ok(TransitionResult {
            state: DidExchangeService::from_parts(
                state,
                their_ddo,
                PairwiseInfo {
                    pw_did: peer_did.to_string(),
                    pw_vk: pairwise_info.pw_vk,
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
            self.did_document,
            self.pairwise_info,
        ))
    }
}

impl DidExchangeServiceResponder<Completed> {
    pub fn to_record(self) -> ConnectionRecord {
        ConnectionRecord::from_parts(self.did_document, self.pairwise_info)
    }
}
