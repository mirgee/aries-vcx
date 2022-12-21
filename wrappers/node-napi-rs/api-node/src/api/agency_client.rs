use napi_derive::napi;

use vcx::api_vcx::api_global::agency_client;
use vcx::api_vcx::api_global::settings::enable_mocks;
use vcx::api_vcx::api_handle::mediated_connection::update_message_status;
use vcx::aries_vcx::agency_client::configuration::{AgencyClientConfig, AgentProvisionConfig};
use vcx::aries_vcx::agency_client::messages::update_message::UIDsByConn;
use vcx::aries_vcx::agency_client::MessageStatusCode;
use vcx::aries_vcx::utils::test_logger::LibvcxDefaultLogger;
use vcx::errors::error::{LibvcxError, LibvcxErrorKind};
use vcx::serde_json;

use crate::error::to_napi_err;

#[napi]
pub fn create_agency_client_for_main_wallet(config: String) -> napi::Result<()> {
    let config = serde_json::from_str::<AgencyClientConfig>(&config)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    agency_client::create_agency_client_for_main_wallet(&config).map_err(to_napi_err)?;
    Ok(())
}

#[napi]
pub async fn provision_cloud_agent(config: String) -> napi::Result<()> {
    let config = serde_json::from_str::<AgentProvisionConfig>(&config)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    agency_client::provision_cloud_agent(&config).await.map_err(to_napi_err)?;
    Ok(())
}

// todo: can we accept Vec<String> instead of Stringified JSON in place of uids_by_conns?
#[napi]
pub async fn messages_update_status(status_code: String, uids_by_conns: String) -> napi::Result<()> {
    let status_code = serde_json::from_str::<MessageStatusCode>(&status_code)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    let uids_by_conns = serde_json::from_str::<Vec<UIDsByConn>>(&uids_by_conns)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;

    agency_client::agency_update_messages(status_code, uids_by_conns)
        .await
        .map_err(to_napi_err)?;
    Ok(())
}
