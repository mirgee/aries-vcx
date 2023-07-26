use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::extra_fields::aip1::ExtraFieldsAIP1;
use did_doc_sov::extra_fields::didcommv1::ExtraFieldsDidCommV1;
use did_doc_sov::extra_fields::KeyKind;
use did_doc_sov::service::aip1::ServiceAIP1;
use did_doc_sov::service::didcommv1::ServiceDidCommV1;
use did_doc_sov::service::ServiceSov;
use did_doc_sov::DidDocumentSov;
use did_parser::Did;
use diddoc_legacy::aries::diddoc::AriesDidDoc;
use diddoc_legacy::aries::service::AriesService;

use crate::errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult};
use crate::utils::encryption_envelope::EncryptionEnvelope;
use messages::AriesMessage;

#[macro_use]
#[cfg(feature = "vdrtools")]
pub mod devsetup;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{
        $val
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{
        "_"
    }};
}

#[cfg(test)]
macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[rustfmt::skip]
pub mod constants;
pub mod file;
pub mod mockdata;
pub mod openssl;
pub mod provision;
pub mod qualifier;
pub mod random;
pub mod uuid;

#[macro_use]
pub mod test_logger;
pub mod encryption_envelope;
pub mod filters;
pub mod serialization;
pub mod validation;

pub async fn send_message(
    wallet: Arc<dyn BaseWallet>,
    sender_verkey: String,
    did_doc: AriesDidDoc,
    message: AriesMessage,
) -> VcxResult<()> {
    trace!("send_message >>> message: {:?}, did_doc: {:?}", message, &did_doc);
    let EncryptionEnvelope(envelope) =
        EncryptionEnvelope::create(&wallet, &message, Some(&sender_verkey), &did_doc).await?;

    // TODO: Extract from agency client
    agency_client::httpclient::post_message(
        envelope,
        did_doc
            .get_endpoint()
            .ok_or_else(|| AriesVcxError::from_msg(AriesVcxErrorKind::InvalidUrl, "No URL in DID Doc"))?,
    )
    .await?;
    Ok(())
}

pub async fn send_message_anonymously(
    wallet: Arc<dyn BaseWallet>,
    did_doc: &AriesDidDoc,
    message: &AriesMessage,
) -> VcxResult<()> {
    trace!(
        "send_message_anonymously >>> message: {:?}, did_doc: {:?}",
        message,
        &did_doc
    );
    let EncryptionEnvelope(envelope) = EncryptionEnvelope::create(&wallet, message, None, did_doc).await?;

    agency_client::httpclient::post_message(
        envelope,
        did_doc
            .get_endpoint()
            .ok_or_else(|| AriesVcxError::from_msg(AriesVcxErrorKind::InvalidUrl, "No URL in DID Doc"))?,
    )
    .await?;
    Ok(())
}

pub fn from_did_doc_sov_to_legacy(ddo: DidDocumentSov) -> VcxResult<AriesDidDoc> {
    println!("Converting DID Doc to legacy format {:?}", ddo);
    let mut new_ddo = AriesDidDoc::default();
    new_ddo.id = ddo.id().to_string();
    new_ddo.set_service_endpoint(
        ddo.service()
            .first()
            .ok_or_else(|| AriesVcxError::from_msg(AriesVcxErrorKind::InvalidState, "No service present in DDO"))?
            .service_endpoint()
            .clone()
            .into(),
    );
    if let Some(vm) = ddo.verification_method().first() {
        new_ddo.set_recipient_keys(vec![vm.public_key().base58()?]);
    }
    Ok(new_ddo)
}

pub fn from_legacy_did_doc_to_sov(ddo: AriesDidDoc) -> VcxResult<DidDocumentSov> {
    let did: Did = ddo.id.parse().unwrap_or_default();
    let vm = VerificationMethod::builder(
        did.clone().into(),
        did.clone(),
        VerificationMethodType::Ed25519VerificationKey2018,
    )
    .add_public_key_base58(ddo.recipient_keys()?.first().unwrap().to_string())
    .build();
    let new_ddo = DidDocumentSov::builder(did.clone())
        .add_service(from_legacy_service_to_service_sov(ddo.service.first().unwrap().clone()).unwrap())
        .add_controller(did.clone())
        .add_verification_method(vm)
        .build();
    Ok(new_ddo)
}

pub fn from_legacy_service_to_service_sov(service: AriesService) -> VcxResult<ServiceSov> {
    let extra = ExtraFieldsDidCommV1::builder()
        .set_recipient_keys(
            service
                .recipient_keys
                .iter()
                .map(String::to_owned)
                .map(KeyKind::Value)
                .collect(),
        )
        .set_routing_keys(
            service
                .routing_keys
                .iter()
                .map(String::to_owned)
                .map(KeyKind::Value)
                .collect(),
        )
        .build();
    Ok(ServiceSov::DIDCommV1(ServiceDidCommV1::new(
        service.id.parse().unwrap_or_default(),
        service.service_endpoint.into(),
        extra,
    )?))
}

pub fn from_service_sov_to_legacy(service: ServiceSov) -> AriesService {
    match service {
        ServiceSov::AIP1(service) => AriesService {
            id: service.id().to_string(),
            service_endpoint: service.service_endpoint().clone().into(),
            ..Default::default()
        },
        ServiceSov::DIDCommV1(service) => {
            let extra = service.extra();
            let recipient_keys = extra.recipient_keys().iter().map(|key| key.to_string()).collect();
            let routing_keys = extra.routing_keys().iter().map(|key| key.to_string()).collect();
            AriesService {
                id: service.id().to_string(),
                recipient_keys,
                routing_keys,
                service_endpoint: service.service_endpoint().clone().into(),
                ..Default::default()
            }
        }
        ServiceSov::DIDCommV2(service) => {
            let extra = service.extra();
            let routing_keys = extra.routing_keys().iter().map(|key| key.to_string()).collect();
            AriesService {
                id: service.id().to_string(),
                routing_keys,
                service_endpoint: service.service_endpoint().clone().into(),
                ..Default::default()
            }
        }
    }
}
