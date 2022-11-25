use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi]
pub fn mediated_connection_generate_public_invite(_public_did: String, _label: String) -> napi::Result<String> {
    todo!()
}

#[napi]
pub fn mediated_connection_get_pw_did(_handle: u32) -> napi::Result<String> {
    todo!()
}

#[napi]
pub fn mediated_connection_get_their_pw_did(_handle: u32) -> napi::Result<String> {
    todo!()
}

#[napi]
pub fn mediated_connection_get_thread_id(_handle: u32) -> napi::Result<String> {
    todo!()
}

#[napi]
pub fn mediated_connection_get_state(_handle: u32) -> u32 {
    todo!()
}

#[napi]
pub fn mediated_connection_get_source_id(_handle: u32) -> napi::Result<String> {
    todo!()
}

#[napi]
pub async fn mediated_connection_create(_source_id: String) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_create_with_invite(_source_id: String, _details: String) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_create_with_connection_request(_request: String, _agent_handle: u32) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_create_with_connection_request_v2(_request: String, _pw_info: String) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_send_message(_handle: u32, _msg: String) -> napi::Result<()> {
    todo!()
}

#[napi]
pub async fn mediated_connection_send_handshake_reuse(_handle: u32, _oob_msg: String) -> napi::Result<()> {
    todo!()
}

#[napi]
pub async fn mediated_connection_update_state_with_message(_handle: u32, _message: String) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_handle_message(_handle: u32, _message: String) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_update_state(_handle: u32) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_delete_connection(_handle: u32) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub async fn mediated_connection_connect(_handle: u32) -> napi::Result<Option<String>> {
    todo!()
}

#[napi]
pub fn mediated_connection_serialize(_handle: u32) -> napi::Result<String> {
    todo!()
}

#[napi]
pub fn mediated_connection_deserialize(_connection_data: String) -> napi::Result<u32> {
    todo!()
}

#[napi]
pub fn mediated_connection_release(_handle: u32) -> napi::Result<()> {
    todo!()
}

#[napi]
pub fn mediated_connection_invite_details(_handle: u32) -> napi::Result<String> {
    todo!()
}

#[napi]
pub async fn mediated_connection_send_ping(_handle: u32, _comment: Option<String>) -> napi::Result<()> {
    todo!()
}

#[napi]
pub async fn mediated_connection_send_discovery_features(_handle: u32, _query: Option<String>, _comment: Option<String>) -> napi::Result<()> {
    todo!()
}

#[napi]
pub async fn mediated_connection_info(_handle: u32) -> napi::Result<String> {
    todo!()
}

#[napi]
pub async fn mediated_connection_messages_download(
    _conn_handles: Vec<u32>,
    _status_codes: Option<String>,
    _uids: Option<String>,
) -> napi::Result<String> {
    todo!()
}

#[napi]
pub async fn mediated_connection_sign_data(_handle: u32, _data: Buffer) -> napi::Result<Buffer> {
    todo!()
}

#[napi]
pub async fn mediated_connection_verify_signature(_handle: u32, _data: Buffer, _signature: Buffer) -> napi::Result<bool> {
    todo!()
}
