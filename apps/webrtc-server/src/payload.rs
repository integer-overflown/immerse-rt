use std::fmt::Display;

use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Eq, PartialEq)]
pub enum PeerRole {
    Publisher,
    Subscriber,
}

impl PeerRole {
    pub fn can_publish(&self) -> bool {
        *self == PeerRole::Publisher
    }

    pub fn can_subscribe(&self) -> bool {
        *self == PeerRole::Subscriber
    }

    fn expected_value_message() -> &'static str {
        r#"a valid peer role: "PUBLISHER" or "SUBSCRIBER""#
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    pub identity: String,
    pub name: Option<String>,
    pub room_id: String,
    pub role: PeerRole,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub token: String,
}

impl Display for PeerRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeerRole::Publisher => write!(f, "PUBLISHER"),
            PeerRole::Subscriber => write!(f, "SUBSCRIBER"),
        }
    }
}

impl TryFrom<&str> for PeerRole {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "PUBLISHER" => Ok(PeerRole::Publisher),
            "SUBSCRIBER" => Ok(PeerRole::Subscriber),
            _ => Err(()),
        }
    }
}

impl Serialize for PeerRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PeerRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let name: &str = Deserialize::deserialize(deserializer)?;
        PeerRole::try_from(name).map_err(|_| {
            Error::invalid_value(Unexpected::Str(name), &PeerRole::expected_value_message())
        })
    }
}
