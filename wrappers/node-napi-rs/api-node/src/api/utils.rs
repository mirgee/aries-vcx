use napi_derive::napi;

use vcx::api_vcx::api_global::settings::enable_mocks;
use vcx::api_vcx::api_global::state::state_vcx_shutdown;
use vcx::api_vcx::api_global::VERSION_STRING;

use crate::error::to_napi_err;

#[napi]
pub fn shutdown(delete_all: Option<bool>) -> ::napi::Result<()> {
    state_vcx_shutdown(delete_all.unwrap_or(false));
    Ok(())
}

#[napi]
pub fn get_version() -> ::napi::Result<String> {
    return Ok(VERSION_STRING.clone());
}
