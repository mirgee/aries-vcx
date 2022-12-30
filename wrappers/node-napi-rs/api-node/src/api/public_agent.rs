use napi_derive::napi;

use vcx::api_vcx::api_handle::{agent, out_of_band};

use crate::error::to_napi_err;

fn parse_uids_arg(uids: String) -> Vec<String> {
    let v: Vec<&str> = uids.split(',').collect();
    v.iter().map(|s| s.to_string()).collect()
}

#[napi]
pub async fn public_agent_create(source_id: String, institution_did: String) -> napi::Result<u32> {
    agent::create_public_agent(&source_id, &institution_did)
        .await
        .map_err(to_napi_err)
}

#[napi]
pub async fn public_agent_download_connection_requests(handle: u32, uids: Option<String>) -> napi::Result<String> {
    agent::download_connection_requests(handle, uids.map(parse_uids_arg).as_ref())
        .await
        .map_err(to_napi_err)
}

#[napi]
pub async fn public_agent_download_message(handle: u32, uid: String) -> napi::Result<String> {
    agent::download_message(handle, &uid).await.map_err(to_napi_err)
}

#[napi]
pub fn public_agent_get_service(handle: u32) -> napi::Result<String> {
    agent::get_service(handle).map_err(to_napi_err)
}

#[napi]
pub fn public_agent_serialize(handle: u32) -> napi::Result<String> {
    agent::to_string(handle).map_err(to_napi_err)
}

#[napi]
pub fn public_agent_deserialize(data: String) -> napi::Result<u32> {
    agent::from_string(&data).map_err(to_napi_err)
}

#[napi]
pub fn public_agent_release(handle: u32) -> napi::Result<()> {
    agent::release(handle).map_err(to_napi_err)
}
