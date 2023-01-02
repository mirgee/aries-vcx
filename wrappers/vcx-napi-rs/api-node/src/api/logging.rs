use napi_derive::napi;
use vcx::api_vcx::api_global::agency_client::create_agency_client_for_main_wallet;
use vcx::api_vcx::api_global::settings::enable_mocks;
use vcx::aries_vcx::agency_client::configuration::AgencyClientConfig;
use vcx::aries_vcx::utils::test_logger::LibvcxDefaultLogger;
use vcx::errors::error::{LibvcxError, LibvcxErrorKind};
use vcx::serde_json;

use crate::error::{ariesvcx_to_napi_err, to_napi_err};

#[napi]
pub fn init_default_logger(pattern: Option<String>) -> napi::Result<()> {
    LibvcxDefaultLogger::init(pattern).map_err(ariesvcx_to_napi_err)
}
