use std::sync::Arc;

use aries_vcx::{
    core::profile::profile::Profile,
    did_doc_sov::{
        extra_fields::{didcommv1::ExtraFieldsDidCommV1, didcommv2::ExtraFieldsDidCommV2, KeyKind},
        service::{didcommv1::ServiceDidCommV1, didcommv2::ServiceDidCommV2, ServiceSov},
    },
    handlers::out_of_band::{receiver::OutOfBandReceiver, sender::OutOfBandSender, GenericOutOfBand},
    messages::{
        msg_fields::protocols::out_of_band::invitation::{Invitation as OobInvitation, OobService},
        AriesMessage,
    },
    protocols::mediated_connection::pairwise_info::PairwiseInfo,
};
use uuid::Uuid;

use crate::{
    storage::{object_cache::ObjectCache, Storage},
    AgentResult,
};

use super::connection::ServiceEndpoint;

pub struct ServiceOutOfBand {
    profile: Arc<dyn Profile>,
    service_endpoint: ServiceEndpoint,
    out_of_band: Arc<ObjectCache<GenericOutOfBand>>,
}

impl ServiceOutOfBand {
    pub fn new(profile: Arc<dyn Profile>, service_endpoint: ServiceEndpoint) -> Self {
        Self {
            profile,
            service_endpoint,
            out_of_band: Arc::new(ObjectCache::new("out-of-band")),
        }
    }

    pub async fn create_invitation(&self) -> AgentResult<AriesMessage> {
        let pw_info = PairwiseInfo::create(&self.profile.inject_wallet()).await?;
        let service = {
            let service_id = Uuid::new_v4().to_string();
            ServiceSov::DIDCommV1(ServiceDidCommV1::new(
                service_id.parse()?,
                self.service_endpoint.to_owned().into(),
                ExtraFieldsDidCommV1::builder()
                    .set_recipient_keys(vec![KeyKind::Value(pw_info.pw_vk)])
                    .build(),
            )?)
        };
        let sender = OutOfBandSender::create().append_service(&OobService::SovService(service));

        self.out_of_band
            .insert(&sender.get_id(), GenericOutOfBand::Sender(sender.to_owned()))?;

        Ok(sender.to_aries_message())
    }

    pub fn receive_invitation(&self, invitation: AriesMessage) -> AgentResult<String> {
        let receiver = OutOfBandReceiver::create_from_a2a_msg(&invitation)?;

        let id = receiver.get_id();
        self.out_of_band.insert(&id, GenericOutOfBand::Receiver(receiver))?;

        Ok(id)
    }

    pub fn get_invitation(&self, invitation_id: &str) -> AgentResult<OobInvitation> {
        let out_of_band = self.out_of_band.get(invitation_id)?;
        match out_of_band {
            GenericOutOfBand::Sender(sender) => Ok(sender.oob.clone()),
            GenericOutOfBand::Receiver(receiver) => Ok(receiver.oob.clone()),
        }
    }

    pub fn exists_by_id(&self, thread_id: &str) -> bool {
        self.out_of_band.contains_key(thread_id)
    }
}
