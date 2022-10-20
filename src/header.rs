use axon_protocol::types::{Hash, Header as AxonHeader};
use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc_proto::google::protobuf::Any;
use serde::{Deserialize, Serialize};

pub const AXON_HEADER_TYPE_URL: &str = "/ibc.lightclients.axon.v1.Header";

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Header(AxonHeader);

impl Header {
    pub fn get_height(&self) -> u64 {
        self.0.number
    }

    pub fn get_prev_hash(&self) -> Hash {
        self.0.prev_hash
    }
}

impl TryFrom<Any> for Header {
    type Error = Ics02Error;

    fn try_from(value: Any) -> Result<Self, Self::Error> {
        todo!()
    }
}
