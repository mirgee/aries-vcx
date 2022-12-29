use napi_derive::napi;

use crate::error::to_napi_err;
use vcx::api_vcx::api_global::ledger::{ledger_get_txn_author_agreement, ledger_set_txn_author_agreement};

#[napi]
async fn get_ledger_author_agreement() -> napi::Result<String> {
    let res = ledger_get_txn_author_agreement().await.map_err(to_napi_err)?;
    Ok(res)
}

// todo: ideally time_of_acceptance is u64, but napi doesn't support u64
#[napi]
fn set_active_txn_author_agreement_meta(
    text: Option<String>,
    version: Option<String>,
    hash: Option<String>,
    acc_mech_type: String,
    time_of_acceptance: u32,
) -> napi::Result<()> {
    ledger_set_txn_author_agreement(text, version, hash, acc_mech_type, time_of_acceptance as u64).map_err(to_napi_err)
}
