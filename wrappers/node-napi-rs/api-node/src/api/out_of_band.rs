use napi_derive::napi;
use ::vcx::api_lib::api_handle::out_of_band;

use crate::error::to_napi_err;

#[napi]
// TODO: Does not need to be async
pub async fn out_of_band_sender_create(config: String) -> napi::Result<u32> {
    out_of_band::create_out_of_band(&config)
        .await
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_receiver_create(msg: String) -> napi::Result<u32> {
    out_of_band::create_out_of_band_msg_from_msg(&msg)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_append_message(handle: u32, msg: String) -> napi::Result<()> {
    out_of_band::append_message(handle, &msg)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_append_service(handle: u32, service: String) -> napi::Result<()> {
    out_of_band::append_service(handle, &service)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_append_service_did(handle: u32, did: String) -> napi::Result<()> {
    out_of_band::append_service_did(handle, &did)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_receiver_extract_message(handle: u32) -> napi::Result<String> {
    out_of_band::extract_a2a_message(handle)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_to_message(handle: u32) -> napi::Result<String> {
    out_of_band::to_a2a_message(handle)
        .map_err(to_napi_err)
}

#[napi]
pub async fn out_of_band_receiver_connection_exists(handle: u32, conn_handles: Vec<u32>) -> napi::Result<u32> {
    out_of_band::connection_exists(handle, &conn_handles)
        .await
        .map(|res| res.0)
        .map_err(to_napi_err)
}

#[napi]
pub async fn out_of_band_receiver_build_connection(handle: u32) -> napi::Result<String> {
    out_of_band::build_connection(handle)
        .await
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_get_thread_id(handle: u32) -> napi::Result<String> {
    out_of_band::get_thread_id_sender(handle)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_receiver_get_thread_id(handle: u32) -> napi::Result<String> {
    out_of_band::get_thread_id_receiver(handle)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_serialize(handle: u32) -> napi::Result<String> {
    out_of_band::to_string_sender(handle)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_receiver_serialize(handle: u32) -> napi::Result<String> {
    out_of_band::to_string_receiver(handle)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_deserialize(oob_data: String) -> napi::Result<u32> {
    out_of_band::from_string_sender(&oob_data)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_receiver_deserialize(oob_data: String) -> napi::Result<u32> {
    out_of_band::from_string_receiver(&oob_data)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_sender_release(handle: u32) -> napi::Result<()> {
    out_of_band::release_sender(handle)
        .map_err(to_napi_err)
}

#[napi]
pub fn out_of_band_receiver_release(handle: u32) -> napi::Result<()> {
    out_of_band::release_receiver(handle)
        .map_err(to_napi_err)
}
