use vcx::api_lib::utils::logger::VcxError;

pub fn to_napi_err(err: VcxError) -> napi::Error {
    error!("{}", err.to_string());
    napi::Error::new(napi::Status::Unknown, format!("{:?}", Into::<u32>::into(err.kind())))
}
