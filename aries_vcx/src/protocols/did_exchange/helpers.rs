use did_doc_sov::DidDocumentSov;
use diddoc_legacy::aries::diddoc::AriesDidDoc;
use messages::decorators::attachment::{Attachment, AttachmentData, AttachmentType};

use crate::{
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    utils::from_legacy_did_doc_to_sov,
};

pub fn ddo_sov_to_attach(ddo: DidDocumentSov) -> Attachment {
    // TODO: Use b64
    Attachment::new(AttachmentData::new(AttachmentType::Json(
        serde_json::to_value(&ddo).unwrap(),
    )))
}

pub fn attach_to_ddo_sov(attachment: Attachment) -> Result<DidDocumentSov, AriesVcxError> {
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
                return Ok(ddo);
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
