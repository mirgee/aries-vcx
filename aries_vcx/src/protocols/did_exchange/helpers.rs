use did_doc_sov::DidDocumentSov;
use messages::decorators::attachment::{Attachment, AttachmentData, AttachmentType};

use crate::errors::error::{AriesVcxError, AriesVcxErrorKind};

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
        _ => Err(AriesVcxError::from_msg(
            AriesVcxErrorKind::InvalidJson,
            "Attachment is not a JSON",
        )),
    }
}
