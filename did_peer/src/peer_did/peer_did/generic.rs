use crate::{
    error::DidPeerError,
    peer_did::{
        numalgos::{numalgo2::Numalgo2, numalgo3::Numalgo3, NumalgoKind},
        parse::parse_numalgo,
        validate::validate,
    },
};
use did_parser::Did;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::PeerDid;

#[derive(Clone, Debug, PartialEq)]
pub enum GenericPeerDid {
    Numalgo2(PeerDid<Numalgo2>),
    Numalgo3(PeerDid<Numalgo3>),
}

impl GenericPeerDid {
    pub fn parse<T>(did: T) -> Result<GenericPeerDid, DidPeerError>
    where
        Did: TryFrom<T>,
        <Did as TryFrom<T>>::Error: Into<DidPeerError>,
    {
        let did: Did = did.try_into().map_err(Into::into)?;
        let numalgo = parse_numalgo(&did)?;
        validate(&did)?;
        let parsed = match numalgo {
            NumalgoKind::MultipleInceptionKeys(numalgo) => GenericPeerDid::Numalgo2(PeerDid { did, numalgo }),
            _ => GenericPeerDid::Numalgo3(PeerDid { did, numalgo: Numalgo3 }),
        };
        Ok(parsed)
    }

    pub fn numalgo(&self) -> NumalgoKind {
        match self {
            GenericPeerDid::Numalgo2(peer_did) => NumalgoKind::MultipleInceptionKeys(peer_did.numalgo),
            GenericPeerDid::Numalgo3(peer_did) => NumalgoKind::DidShortening(peer_did.numalgo),
        }
    }
}

impl Serialize for GenericPeerDid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            GenericPeerDid::Numalgo2(peer_did) => serializer.serialize_str(peer_did.did().did()),
            GenericPeerDid::Numalgo3(peer_did) => serializer.serialize_str(peer_did.did().did()),
        }
    }
}

impl<'de> Deserialize<'de> for GenericPeerDid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let did = String::deserialize(deserializer)?;
        Self::parse(did).map_err(serde::de::Error::custom)
    }
}
