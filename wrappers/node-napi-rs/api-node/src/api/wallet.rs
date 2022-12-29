use napi_derive::napi;
use vcx::api_vcx::api_global::settings::settings_init_issuer_config;
use vcx::api_vcx::api_global::wallet::{
    close_main_wallet, create_main_wallet, open_as_main_wallet, wallet_configure_issuer,
};
use vcx::aries_vcx::indy::wallet::{IssuerConfig, WalletConfig};
use vcx::errors::error::{LibvcxError, LibvcxErrorKind};
use vcx::serde_json;
use vcx::serde_json::json;

use crate::error::to_napi_err;

#[napi]
pub async fn wallet_open_as_main(wallet_config: String) -> napi::Result<i32> {
    let wallet_config = serde_json::from_str::<WalletConfig>(&wallet_config)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    let handle = open_as_main_wallet(&wallet_config).await.map_err(to_napi_err)?;
    Ok(handle.0)
}

#[napi]
pub async fn wallet_create_main(wallet_config: String) -> napi::Result<()> {
    let wallet_config = serde_json::from_str::<WalletConfig>(&wallet_config)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    create_main_wallet(&wallet_config).await.map_err(to_napi_err)
}

#[napi]
pub async fn wallet_close_main() -> napi::Result<()> {
    close_main_wallet().await.map_err(to_napi_err)
}

#[napi]
pub async fn vcx_init_issuer_config(config: String) -> napi::Result<()> {
    let config = serde_json::from_str::<IssuerConfig>(&config)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    settings_init_issuer_config(&config).map_err(to_napi_err)
}

#[napi]
pub async fn configure_issuer_wallet(enterprise_seed: String) -> napi::Result<String> {
    let res = wallet_configure_issuer(&enterprise_seed).await.map_err(to_napi_err)?;
    Ok(json!(res).to_string())
}
