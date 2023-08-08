use std::sync::Arc;

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

pub async fn generate_keypair(wallet: &Arc<dyn BaseWallet>, key_type: KeyType) -> Result<Key, AriesVcxError> {
    let pairwise_info = PairwiseInfo::create(wallet).await?;
    Ok(Key::from_base58(&pairwise_info.pw_vk, key_type)?)
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

pub async fn create_our_did_document(
    wallet: &Arc<dyn BaseWallet>,
    service_endpoint: Url,
    routing_keys: Vec<String>,
) -> Result<(DidDocumentSov, Key), AriesVcxError> {
    let key_ver = generate_keypair(wallet, KeyType::Ed25519).await?;
    let key_enc = generate_keypair(wallet, KeyType::X25519).await?;
    let service = construct_service(
        routing_keys.into_iter().map(KeyKind::Value).collect(),
        vec![KeyKind::DidKey(key_enc.clone().try_into()?)],
        service_endpoint,
    )?;

    // TODO: Make it easier to generate peer did from keys and service, and generate DDO from it
    let did_document_temp = did_doc_from_keys(Default::default(), key_ver.clone(), key_enc.clone(), service.clone());
    let peer_did = generate_numalgo2(did_document_temp.into())?;

    Ok((
        did_doc_from_keys(peer_did.clone().into(), key_ver, key_enc.clone(), service),
        key_enc,
    ))
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

pub fn ddo_sov_to_attach(ddo: DidDocumentSov) -> Result<Attachment, AriesVcxError> {
    // TODO: Use b64, more compact
    // TODO: DDO attachment must be signed!
    // Interop note: acapy accepts unsigned
    Ok(Attachment::new(AttachmentData::new(AttachmentType::Json(
        serde_json::to_value(&ddo)?,
    ))))
}

pub fn attach_to_ddo_sov(attachment: Attachment) -> Result<DidDocumentSov, AriesVcxError> {
    // TODO: Support more attachment types
    match attachment.data.content {
        AttachmentType::Json(value) => serde_json::from_value(value).map_err(Into::into),
        AttachmentType::Base64(ref value) => {
            let bytes = base64::decode(&value).map_err(|err| {
                AriesVcxError::from_msg(
                    AriesVcxErrorKind::SerializationError,
                    format!("Attachment base 64 decoding failed; attach: {attachment:?}, err: {err}"),
                )
            })?;
            // TODO: Try to make DidDocumentSov support the legacy DDO if possible - would make
            // integration of DidDocument much easier
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
            "Attachment is not a JSON or Base64",
        )),
    }
}
