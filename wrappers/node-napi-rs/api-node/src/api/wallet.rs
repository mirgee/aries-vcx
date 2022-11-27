use napi_derive::napi;
use vcx::{
    api_lib::global::wallet::{
        create_main_wallet, get_main_wallet_handle, reset_main_wallet_handle,
        set_main_wallet_handle,
    },
    aries_vcx::{
        error::{VcxError, VcxErrorKind},
        indy::{self, wallet::WalletConfig},
    },
    serde_json,
};

use crate::error::to_napi_err;

#[napi]
pub async fn wallet_open_as_main(wallet_config: String) -> napi::Result<i32> {
    let wallet_config = serde_json::from_str::<WalletConfig>(&wallet_config)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    let handle = indy::wallet::open_wallet(&wallet_config).await.map_err(to_napi_err)?;
    set_main_wallet_handle(handle);
    Ok(handle.0)
}

#[napi]
pub async fn wallet_create_main(wallet_config: String) -> napi::Result<()> {
    let wallet_config = serde_json::from_str::<WalletConfig>(&wallet_config)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    create_main_wallet(&wallet_config)
        .await
        .map_err(to_napi_err)
}

#[napi]
pub async fn wallet_close_main() -> ::napi::Result<()> {
    indy::wallet::close_wallet(get_main_wallet_handle())
        .await
        .map_err(to_napi_err)?;
    reset_main_wallet_handle();
    Ok(())
}
