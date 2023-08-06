#[macro_use]
extern crate log;
mod fixtures;
mod utils;

use aries_vcx::protocols::did_exchange::service::requester::DidExchangeServiceRequester;
use aries_vcx::protocols::did_exchange::service::responder::DidExchangeServiceResponder;
use aries_vcx::protocols::did_exchange::states::requester::request_sent::RequestSent;
use aries_vcx::protocols::did_exchange::states::responder::response_sent::ResponseSent;
use aries_vcx::protocols::did_exchange::transition::transition_result::TransitionResult;
use aries_vcx::utils::devsetup::SetupPoolDirectory;
use did_doc::schema::verification_method::PublicKeyField;
use did_doc_sov::extra_fields::didcommv2::ExtraFieldsDidCommV2;
use did_doc_sov::service::didcommv2::ServiceDidCommV2;
use did_doc_sov::service::ServiceSov;
use messages::msg_fields::protocols::out_of_band::invitation::Invitation;
use url::Url;

use crate::utils::devsetup_alice::create_alice;
use crate::utils::devsetup_faber::create_faber;

// #[tokio::test]
// async fn did_exchange_test() {
//     SetupPoolDirectory::run(|setup| async move {
//         let institution = create_faber(setup.genesis_file_path.clone()).await;
//         let consumer = create_alice(setup.genesis_file_path).await;
//
//         let url: Url = "http://dummyurl.org".parse().unwrap();
//         let invitation: Invitation = serde_json::from_str(fixtures::OOB_INVITE).unwrap();
//         let invitation_id = invitation.id.clone();
//         let TransitionResult {
//             state: requester,
//             output: request,
//         } = DidExchangeServiceRequester::<RequestSent>::construct_request_pairwise(
//             &consumer.profile.inject_wallet(),
//             invitation,
//             url.clone(),
//             vec![],
//         )
//         .await
//         .unwrap();
//
//         let extra = ExtraFieldsDidCommV2::builder().build();
//         let service = ServiceSov::DIDCommV2(ServiceDidCommV2::new(Default::default(), url.into(), extra).unwrap());
//         let TransitionResult {
//             state: responder,
//             output: response,
//         } = DidExchangeServiceResponder::<ResponseSent>::receive_request(
//             &institution.profile.inject_wallet(),
//             request,
//             service,
//             invitation_id,
//         )
//         .await
//         .unwrap();
//
//         let TransitionResult {
//             state: requester,
//             output: complete,
//         } = requester.receive_response(response).await.unwrap();
//
//         let responder = responder.receive_complete(complete).unwrap();
//
//         let record_requester = requester.to_record();
//         let record_responder = responder.to_record();
//
//         let responder_key = match record_responder.did_document().verification_method()[0].public_key() {
//             PublicKeyField::Base58 { public_key_base58 } => public_key_base58.clone(),
//             _ => panic!("Unexpected key type"),
//         };
//         assert_eq!(record_requester.pairwise_info().pw_vk, responder_key);
//
//         let requester_key = match record_requester.did_document().verification_method()[0].public_key() {
//             PublicKeyField::Base58 { public_key_base58 } => public_key_base58.clone(),
//             _ => panic!("Unexpected key type"),
//         };
//         assert_eq!(record_responder.pairwise_info().pw_vk, requester_key);
//     })
//     .await;
// }
