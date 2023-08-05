pub mod generic;
pub mod requester;
pub mod responder;

use std::sync::Arc;

use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{
    extra_fields::{didcommv1::ExtraFieldsDidCommV1, KeyKind},
    service::{didcommv1::ServiceDidCommV1, ServiceSov},
    DidDocumentSov,
};
use did_parser::Did;
use public_key::{Key, KeyType};
use url::Url;

use crate::{errors::error::AriesVcxError, protocols::mediated_connection::pairwise_info::PairwiseInfo};

use super::protocol::DidExchangeProtocol;

// Should be able to:
// * accept OOB invitation and create didexchange protocol SM, request and hold our pwdid
// * accept request and create didexchange protocol SM, response, and hold DDO of counterparty and our pwdid
// * accept response and transition the existing SM to complete, complete message, and hold the final DDO of counterparty and our pwdid
// The final result should be something which would be eventually uniform for both didexchange and
// connection protocols. I.e. this "service" can be didexchange-specific, but the final outcome
// should be a "ConnectionRecord", which would hold all the information necessary to send messages
// to the counterparty.
// 1.) Should this be two separate structs (requester, responder), or should it
// be a single struct similar to the protocol?
// * What would be shared: thread id and state getters, and the final outcome (ConnectionRecord)
// * The interface will be similar to that of ConnectionRecord in the intermediate states (bootstrap
// vs. final DDO)
// 2.) Should it be generic over the state?
#[derive(Debug, Clone, PartialEq)]
pub struct DidExchangeService<I, S> {
    sm: DidExchangeProtocol<I, S>,
    our_verkey: Key,
    their_did_document: DidDocumentSov,
}

impl<I, S> DidExchangeService<I, S> {
    pub fn our_verkey(&self) -> &Key {
        &self.our_verkey
    }

    pub fn their_did_doc(&self) -> &DidDocumentSov {
        &self.their_did_document
    }
}

impl<I, S> DidExchangeService<I, S> {
    pub fn from_parts(sm: DidExchangeProtocol<I, S>, their_did_document: DidDocumentSov, our_verkey: Key) -> Self {
        Self {
            sm,
            our_verkey,
            their_did_document,
        }
    }
}

pub async fn generate_keypair(wallet: &Arc<dyn BaseWallet>, key_type: KeyType) -> Result<Key, AriesVcxError> {
    let pairwise_info = PairwiseInfo::create(wallet).await?;
    Ok(Key::from_base58(&pairwise_info.pw_vk, key_type).unwrap())
}

pub fn construct_service(
    routing_keys: Vec<KeyKind>,
    recipient_keys: Vec<KeyKind>,
    service_endpoint: Url,
) -> Result<ServiceSov, AriesVcxError> {
    let extra = ExtraFieldsDidCommV1::builder()
        .set_routing_keys(routing_keys)
        .set_recipient_keys(recipient_keys)
        .build();
    let service = ServiceSov::DIDCommV1(ServiceDidCommV1::new(
        Default::default(),
        service_endpoint.into(),
        extra,
    )?);
    Ok(service)
}

pub fn did_doc_from_keys(did: Did, key_ver: Key, key_enc: Key, service: ServiceSov) -> DidDocumentSov {
    let vm_ver = VerificationMethod::builder(
        did.clone().into(),
        did.clone(),
        VerificationMethodType::Ed25519VerificationKey2020,
    )
    .add_public_key_base58(key_ver.base58())
    .build();
    let vm_ka = VerificationMethod::builder(
        did.clone().into(),
        did.clone(),
        VerificationMethodType::X25519KeyAgreementKey2020,
    )
    .add_public_key_base58(key_enc.base58())
    .build();
    DidDocumentSov::builder(did)
        .add_service(service)
        .add_verification_method(vm_ver.clone())
        .add_key_agreement(vm_ka)
        .build()
}
