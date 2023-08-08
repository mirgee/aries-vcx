pub mod generic;
pub mod requester;
pub mod responder;

use std::{marker::PhantomData, sync::Arc};

use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{
    extra_fields::{didcommv1::ExtraFieldsDidCommV1, KeyKind},
    service::{didcommv1::ServiceDidCommV1, ServiceSov},
    DidDocumentSov,
};
use did_parser::Did;
use did_peer::peer_did::generate::generate_numalgo2;
use diddoc_legacy::aries::diddoc::AriesDidDoc;
use messages::decorators::attachment::{Attachment, AttachmentData, AttachmentType};
use public_key::{Key, KeyType};
use url::Url;

use crate::{
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    protocols::mediated_connection::pairwise_info::PairwiseInfo,
    utils::from_legacy_did_doc_to_sov,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DidExchangeService<I, S> {
    state: S,
    initiation_type: PhantomData<I>,
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
    pub fn from_parts(state: S, their_did_document: DidDocumentSov, our_verkey: Key) -> Self {
        Self {
            state,
            initiation_type: PhantomData,
            our_verkey,
            their_did_document,
        }
    }
}

async fn generate_keypair(wallet: &Arc<dyn BaseWallet>, key_type: KeyType) -> Result<Key, AriesVcxError> {
    let pairwise_info = PairwiseInfo::create(wallet).await?;
    Ok(Key::from_base58(&pairwise_info.pw_vk, key_type).unwrap())
}

fn construct_service(
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

async fn create_our_did_document(
    wallet: &Arc<dyn BaseWallet>,
    service_endpoint: Url,
    routing_keys: Vec<String>,
) -> Result<(DidDocumentSov, Key), AriesVcxError> {
    let key_ver = generate_keypair(wallet, KeyType::Ed25519).await?;
    let key_enc = generate_keypair(wallet, KeyType::X25519).await?;
    let service = construct_service(
        routing_keys.into_iter().map(KeyKind::Value).collect(),
        vec![KeyKind::DidKey(key_enc.clone().try_into().unwrap())],
        service_endpoint,
    )?;

    let did_document_temp = did_doc_from_keys(Default::default(), key_ver.clone(), key_enc.clone(), service.clone());
    let peer_did = generate_numalgo2(did_document_temp.into())?;

    Ok((
        did_doc_from_keys(peer_did.clone().into(), key_ver, key_enc.clone(), service),
        key_enc,
    ))
}

fn did_doc_from_keys(did: Did, key_ver: Key, key_enc: Key, service: ServiceSov) -> DidDocumentSov {
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

fn ddo_sov_to_attach(ddo: DidDocumentSov) -> Attachment {
    // TODO: Use b64
    Attachment::new(AttachmentData::new(AttachmentType::Json(
        serde_json::to_value(&ddo).unwrap(),
    )))
}

fn attach_to_ddo_sov(attachment: Attachment) -> Result<DidDocumentSov, AriesVcxError> {
    // TODO: Support more attachment types
    match attachment.data.content {
        AttachmentType::Json(value) => serde_json::from_value(value).map_err(Into::into),
        AttachmentType::Base64(ref value) => {
            let bytes = base64::decode(&value).map_err(|_| {
                AriesVcxError::from_msg(
                    AriesVcxErrorKind::SerializationError,
                    format!("Attachment is not base 64 encoded: {attachment:?}"),
                )
            })?;
            if let Ok(ddo) = serde_json::from_slice::<DidDocumentSov>(&bytes) {
                Ok(ddo)
            } else {
                let res: AriesDidDoc = serde_json::from_slice(&bytes).map_err(|err| {
                    AriesVcxError::from_msg(
                        AriesVcxErrorKind::SerializationError,
                        format!("Attachment is not base 64 encoded JSON: {attachment:?}, err: {err:?}"),
                    )
                })?;
                from_legacy_did_doc_to_sov(res)
            }
        }
        _ => Err(AriesVcxError::from_msg(
            AriesVcxErrorKind::InvalidJson,
            "Attachment is not a JSON",
        )),
    }
}
