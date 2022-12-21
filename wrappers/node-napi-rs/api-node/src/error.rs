use vcx::aries_vcx::errors::error::AriesVcxError;
use vcx::errors::error::LibvcxError;

pub fn to_napi_err(err: LibvcxError) -> napi::Error {
    error!("{}", err.to_string());
    napi::Error::new(napi::Status::Unknown, format!("{:?}", Into::<u32>::into(err.kind())))
}

pub fn ariesvcx_to_napi_err(err: AriesVcxError) -> napi::Error {
    to_napi_err(LibvcxError::from(err))
}
