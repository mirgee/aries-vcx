use napi_derive::napi;
use vcx::{
    api_lib::{
        global::{
            agency_client::{get_main_agency_client, set_main_agency_client},
            wallet::get_main_wallet_handle,
        },
        utils::logger::LibvcxDefaultLogger,
    },
    aries_vcx::{
        agency_client::{configuration::AgencyClientConfig, testing::mocking::enable_agency_mocks},
        error::{VcxError, VcxErrorKind},
        global::settings::enable_indy_mocks,
    },
    serde_json,
};

use crate::error::to_napi_err;

#[napi]
pub fn init_default_logger(pattern: Option<String>) -> napi::Result<()> {
    LibvcxDefaultLogger::init(pattern).map_err(to_napi_err)
}

#[napi]
pub fn create_agency_client_for_main_wallet(config: String) -> napi::Result<()> {
    let config = serde_json::from_str::<AgencyClientConfig>(&config)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    let client = get_main_agency_client()
        .map_err(to_napi_err)?
        .configure(get_main_wallet_handle(), &config)
        .map_err(|err| {
            VcxError::from_msg(
                VcxErrorKind::InvalidJson,
                format!("failed to configure agency client: {:?}", err),
            )
        })
        .map_err(to_napi_err)?;
    set_main_agency_client(client);
    Ok(())
}

#[napi]
pub fn enable_mocks() -> ::napi::Result<()> {
    enable_indy_mocks().map_err(to_napi_err)?;
    enable_agency_mocks();
    Ok(())
}
