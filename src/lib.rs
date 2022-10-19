use axon_protocol::types::Header;
use ibc::{core::ics24_host::identifier::ChainId, Height};

pub struct Client {
    pub chain_id: ChainId,
    pub latest_height: Height,
}
