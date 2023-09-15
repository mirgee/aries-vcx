#![allow(clippy::unwrap_used)]

use aries_vcx_core::anoncreds::base_anoncreds::BaseAnonCreds;
use aries_vcx_core::ledger::base_ledger::{AnoncredsLedgerRead, AnoncredsLedgerWrite};
use aries_vcx_core::ledger::indy::pool::test_utils::get_temp_dir_path;
use std::sync::Arc;
use std::time::Duration;

use crate::common::credentials::encoding::encode_attributes;
use crate::common::primitives::credential_definition::CredentialDef;
use crate::common::primitives::credential_definition::CredentialDefConfigBuilder;
use crate::common::primitives::revocation_registry::RevocationRegistry;
use crate::global::settings;
use crate::utils::constants::TEST_TAILS_URL;

pub async fn create_and_write_test_schema(
    anoncreds: &Arc<dyn BaseAnonCreds>,
    ledger_write: &Arc<dyn AnoncredsLedgerWrite>,
    submitter_did: &str,
    attr_list: &str,
) -> (String, String) {
    let data = attr_list.to_string();
    let schema_name: String = crate::utils::random::generate_random_schema_name();
    let schema_version: String = crate::utils::random::generate_random_schema_version();

    let (schema_id, schema_json) = anoncreds
        .issuer_create_schema(&submitter_did, &schema_name, &schema_version, &data)
        .await
        .unwrap();

    let _response = ledger_write
        .publish_schema(&schema_json, submitter_did, None)
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(1000)).await;
    (schema_id, schema_json)
}

// TODO: Split into schema, cred def, and rev reg creation functions
pub async fn create_and_store_credential_def_and_rev_reg(
    anoncreds: &Arc<dyn BaseAnonCreds>,
    ledger_read: &Arc<dyn AnoncredsLedgerRead>,
    ledger_write: &Arc<dyn AnoncredsLedgerWrite>,
    issuer_did: &str,
    attr_list: &str,
) -> (
    String,
    String,
    String,
    String,
    String,
    String,
    CredentialDef,
    RevocationRegistry,
) {
    let (schema_id, schema_json) = create_and_write_test_schema(anoncreds, ledger_write, issuer_did, attr_list).await;
    let config = CredentialDefConfigBuilder::default()
        .issuer_did(issuer_did)
        .schema_id(&schema_id)
        .tag("1")
        .build()
        .unwrap();
    let cred_def = CredentialDef::create(ledger_read, anoncreds, "1".to_string(), config, true)
        .await
        .unwrap()
        .publish_cred_def(ledger_read, ledger_write)
        .await
        .unwrap();

    let tails_dir = String::from(get_temp_dir_path().as_path().to_str().unwrap());

    let mut rev_reg = RevocationRegistry::create(anoncreds, issuer_did, &cred_def.get_cred_def_id(), &tails_dir, 10, 1)
        .await
        .unwrap();
    rev_reg
        .publish_revocation_primitives(ledger_write, TEST_TAILS_URL)
        .await
        .unwrap();

    let cred_def_id = cred_def.get_cred_def_id();
    tokio::time::sleep(Duration::from_millis(1000)).await;
    let cred_def_json = ledger_read.get_cred_def(&cred_def_id, None).await.unwrap();
    (
        schema_id,
        schema_json,
        cred_def_id,
        cred_def_json,
        rev_reg.get_rev_reg_id(),
        tails_dir,
        cred_def,
        rev_reg,
    )
}

async fn create_credential_req(
    anoncreds_issuer: &Arc<dyn BaseAnonCreds>,
    anoncreds_holder: &Arc<dyn BaseAnonCreds>,
    did: &str,
    cred_def_id: &str,
    cred_def_json: &str,
) -> (String, String, String) {
    let offer = anoncreds_issuer
        .issuer_create_credential_offer(cred_def_id)
        .await
        .unwrap();
    let master_secret_name = settings::DEFAULT_LINK_SECRET_ALIAS;
    let (req, req_meta) = anoncreds_holder
        .prover_create_credential_req(&did, &offer, cred_def_json, master_secret_name)
        .await
        .unwrap();
    (offer, req, req_meta)
}

// todo: extract create_and_store_credential_def into caller functions
pub async fn create_and_store_credential(
    anoncreds_issuer: &Arc<dyn BaseAnonCreds>,
    anoncreds_holder: &Arc<dyn BaseAnonCreds>,
    ledger_read: &Arc<dyn AnoncredsLedgerRead>,
    ledger_write: &Arc<dyn AnoncredsLedgerWrite>,
    institution_did: &str,
    attr_list: &str,
) -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    RevocationRegistry,
) {
    let (schema_id, schema_json, cred_def_id, cred_def_json, rev_reg_id, tails_dir, _, rev_reg) =
        create_and_store_credential_def_and_rev_reg(
            anoncreds_issuer,
            ledger_read,
            ledger_write,
            institution_did,
            attr_list,
        )
        .await;

    let offer = anoncreds_issuer
        .issuer_create_credential_offer(&cred_def_id)
        .await
        .unwrap();
    let master_secret_name = settings::DEFAULT_LINK_SECRET_ALIAS;
    let (req, req_meta) = anoncreds_holder
        .prover_create_credential_req(&institution_did, &offer, &cred_def_json, master_secret_name)
        .await
        .unwrap();

    /* create cred */
    let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
    let encoded_attributes = encode_attributes(&credential_data).unwrap();
    let rev_def_json = ledger_read.get_rev_reg_def_json(&rev_reg_id).await.unwrap();

    let (cred, cred_rev_id, _) = anoncreds_issuer
        .issuer_create_credential(
            &offer,
            &req,
            &encoded_attributes,
            Some(rev_reg_id.clone()),
            Some(tails_dir.clone()),
        )
        .await
        .unwrap();
    /* store cred */
    let cred_id = anoncreds_holder
        .prover_store_credential(None, &req_meta, &cred, &cred_def_json, Some(&rev_def_json))
        .await
        .unwrap();
    (
        schema_id,
        schema_json,
        cred_def_id,
        cred_def_json,
        offer,
        req,
        req_meta,
        cred_id,
        rev_reg_id,
        cred_rev_id.unwrap(),
        tails_dir,
        rev_reg,
    )
}

// todo: extract create_and_store_nonrevocable_credential_def into caller functions
pub async fn create_and_store_nonrevocable_credential(
    anoncreds_issuer: &Arc<dyn BaseAnonCreds>,
    anoncreds_holder: &Arc<dyn BaseAnonCreds>,
    ledger_read: &Arc<dyn AnoncredsLedgerRead>,
    ledger_write: &Arc<dyn AnoncredsLedgerWrite>,
    issuer_did: &str,
    attr_list: &str,
) -> (String, String, String, String, String, String, String, String) {
    let (schema_id, schema_json) =
        create_and_write_test_schema(anoncreds_issuer, ledger_write, issuer_did, attr_list).await;

    let config = CredentialDefConfigBuilder::default()
        .issuer_did(issuer_did)
        .schema_id(&schema_id)
        .tag("1")
        .build()
        .unwrap();

    let cred_def = CredentialDef::create(ledger_read, anoncreds_issuer, "1".to_string(), config, false)
        .await
        .unwrap()
        .publish_cred_def(ledger_read, ledger_write)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(1000)).await;

    let cred_def_id = cred_def.get_cred_def_id();
    let cred_def_json = ledger_read.get_cred_def(&cred_def_id, None).await.unwrap();

    let (offer, req, req_meta) = create_credential_req(
        anoncreds_issuer,
        anoncreds_holder,
        issuer_did,
        &cred_def_id,
        &cred_def_json,
    )
    .await;

    /* create cred */
    let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
    let encoded_attributes = encode_attributes(&credential_data).unwrap();

    let (cred, _, _) = anoncreds_issuer
        .issuer_create_credential(&offer, &req, &encoded_attributes, None, None)
        .await
        .unwrap();
    /* store cred */
    let cred_id = anoncreds_holder
        .prover_store_credential(None, &req_meta, &cred, &cred_def_json, None)
        .await
        .unwrap();
    (
        schema_id,
        schema_json,
        cred_def_id,
        cred_def_json,
        offer,
        req,
        req_meta,
        cred_id,
    )
}
