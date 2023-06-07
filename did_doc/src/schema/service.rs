use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::error::DidDocumentBuilderError;

use super::{
    types::{uri::Uri, url::Url},
    utils::OneOrList,
};

type ServiceTypeAlias = OneOrList<String>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service<E>
where
    E: Default,
{
    id: Uri,
    #[serde(rename = "type")]
    service_type: ServiceTypeAlias,
    service_endpoint: Url,
    #[serde(flatten)]
    extra: E,
}

impl<E> Service<E>
where
    E: Default,
{
    pub fn builder(
        id: Uri,
        service_endpoint: Url,
    ) -> Result<ServiceBuilder<E>, DidDocumentBuilderError> {
        ServiceBuilder::new(id, service_endpoint)
    }

    pub fn id(&self) -> &Uri {
        &self.id
    }

    pub fn service_type(&self) -> &ServiceTypeAlias {
        &self.service_type
    }

    pub fn service_endpoint(&self) -> &Url {
        &self.service_endpoint
    }

    pub fn extra(&self) -> &E {
        &self.extra
    }
}

#[derive(Debug)]
pub struct ServiceBuilder<E> {
    id: Uri,
    service_type: HashSet<String>,
    service_endpoint: Url,
    extra: E,
}

impl<E> ServiceBuilder<E>
where
    E: Default,
{
    pub fn new(id: Uri, service_endpoint: Url) -> Result<Self, DidDocumentBuilderError> {
        Ok(Self {
            id,
            service_endpoint,
            service_type: HashSet::new(),
            extra: E::default(),
        })
    }

    pub fn add_service_type(
        mut self,
        service_type: String,
    ) -> Result<Self, DidDocumentBuilderError> {
        if service_type.is_empty() {
            return Err(DidDocumentBuilderError::MissingField("type"));
        }
        self.service_type.insert(service_type);
        Ok(self)
    }

    pub fn add_extra(mut self, extra: E) -> Self {
        self.extra = extra;
        self
    }

    pub fn build(self) -> Result<Service<E>, DidDocumentBuilderError> {
        if self.service_type.is_empty() {
            Err(DidDocumentBuilderError::MissingField("type"))
        } else {
            Ok(Service {
                id: self.id,
                service_type: OneOrList::List(self.service_type.into_iter().collect()),
                service_endpoint: self.service_endpoint,
                extra: self.extra,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_uri() -> Uri {
        Uri::new("http://example.com").unwrap()
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ExtraSov {
        pub priority: u32,
        pub recipient_keys: Vec<String>,
        pub routing_keys: Vec<String>,
    }

    #[test]
    fn test_service_builder_basic() {
        let id = create_valid_uri();
        let service_endpoint = "http://example.com/endpoint";
        let service_type = "DIDCommMessaging".to_string();

        let service =
            ServiceBuilder::<ExtraSov>::new(id.clone(), service_endpoint.try_into().unwrap())
                .unwrap()
                .add_service_type(service_type.clone())
                .unwrap()
                .build()
                .unwrap();

        assert_eq!(service.id(), &id);
        assert_eq!(service.service_endpoint().as_ref(), service_endpoint);
        assert_eq!(service.service_type(), &OneOrList::List(vec![service_type]));
    }

    #[test]
    fn test_service_builder_add_extra() {
        let id = create_valid_uri();
        let service_endpoint = "http://example.com/endpoint";
        let service_type = "DIDCommMessaging".to_string();
        let recipient_keys = vec!["foo".to_string()];
        let routing_keys = vec!["bar".to_string()];
        let extra = ExtraSov {
            priority: 0,
            recipient_keys: recipient_keys.clone(),
            routing_keys: routing_keys.clone(),
        };

        let service = ServiceBuilder::<ExtraSov>::new(id, service_endpoint.try_into().unwrap())
            .unwrap()
            .add_service_type(service_type)
            .unwrap()
            .add_extra(extra)
            .build()
            .unwrap();

        assert_eq!(service.extra().recipient_keys, recipient_keys);
        assert_eq!(service.extra().routing_keys, routing_keys);
    }

    #[test]
    fn test_service_builder_add_duplicate_types() {
        let id = create_valid_uri();
        let service_endpoint = "http://example.com/endpoint";
        let service_type = "DIDCommMessaging".to_string();

        let service = ServiceBuilder::<ExtraSov>::new(id, service_endpoint.try_into().unwrap())
            .unwrap()
            .add_service_type(service_type.clone())
            .unwrap()
            .add_service_type(service_type.clone())
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(service.service_type(), &OneOrList::List(vec![service_type]));
    }

    #[test]
    fn test_service_builder_add_type_missing_type() {
        let id = create_valid_uri();
        let service_endpoint = "http://example.com/endpoint";

        let res = ServiceBuilder::<ExtraSov>::new(id, service_endpoint.try_into().unwrap())
            .unwrap()
            .add_service_type("".to_string());
        assert!(res.is_err());
    }

    #[test]
    fn test_service_serde() {
        let service_serialized = r##"{
          "id": "did:sov:HR6vs6GEZ8rHaVgjg2WodM#did-communication",
          "type": "did-communication",
          "priority": 0,
          "recipientKeys": [
            "did:sov:HR6vs6GEZ8rHaVgjg2WodM#key-agreement-1"
          ],
          "routingKeys": [],
          "accept": [
            "didcomm/aip2;env=rfc19"
          ],
          "serviceEndpoint": "https://example.com/endpoint"
        }"##;

        let service: Service<ExtraSov> = serde_json::from_str(service_serialized).unwrap();
        assert_eq!(
            service.id(),
            &Uri::new("did:sov:HR6vs6GEZ8rHaVgjg2WodM#did-communication").unwrap()
        );
        assert_eq!(
            service.service_type(),
            &OneOrList::One("did-communication".to_string())
        );
        assert_eq!(
            service.service_endpoint().as_ref(),
            "https://example.com/endpoint"
        );
        assert_eq!(service.extra().priority, 0);
        assert_eq!(
            service.extra().recipient_keys,
            vec!["did:sov:HR6vs6GEZ8rHaVgjg2WodM#key-agreement-1".to_string()]
        );
    }
}