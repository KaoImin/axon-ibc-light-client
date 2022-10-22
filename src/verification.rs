use axon_protocol::types::{Bytes, Hasher};
use common_crypto::{BlsPublicKey, BlsSignature, BlsSignatureVerify, HashValue};
use ibc::core::ics02_client::error::Error as Ics02Error;
use overlord::types::{Vote, VoteType};

use crate::header::Header;

// core/consensus/src/adapter.rs: 431L
pub(crate) fn verify_header(header: &Header) -> Result<(), Ics02Error> {
    let block_hash = header.get_proof_block_hash();
    let vote = Vote {
        height:     header.get_proof_height(),
        round:      header.get_proof_round(),
        vote_type:  VoteType::Precommit,
        block_hash: Bytes::from(block_hash.as_bytes().to_vec()),
    };
    let msg = Bytes::from(rlp::encode(&vote));
    let vote_hash = Bytes::from(Hasher::digest(msg).as_bytes().to_vec());
    let hex_pubkey = header
        .get_pub_keys()
        .iter()
        .map(|hex| {
            let pubkey = hex.as_bytes();
            let ret = BlsPublicKey::try_from(pubkey.as_ref())
                .map_err(|e| Ics02Error::client_specific(e.to_string()));
            ret
        })
        .collect::<Result<Vec<_>, _>>()?;

    let aggregate_key = BlsPublicKey::aggregate(hex_pubkey)
        .map_err(|e| Ics02Error::client_specific(e.to_string()))?;
    let aggregated_signature = BlsSignature::try_from(header.get_proof_signature())
        .map_err(|e| Ics02Error::client_specific(e.to_string()))?;
    let hash = HashValue::try_from(vote_hash.as_ref())
        .map_err(|e| Ics02Error::client_specific(e.to_string()))?;

    aggregated_signature
        .verify(&hash, &aggregate_key, &String::from("")) // todo
        .map_err(|e| Ics02Error::client_specific(format!("{}", e.to_string())))?;
    Ok(())
}
