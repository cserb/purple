use account::{Balance, Address};
use itc::Stamp;
use purple_crypto::{Hash, Signature};

pub struct Receive {
    previous_hash: Hash,
    referenced_hash: Hash,
    balance: Balance,
    hash: Option<Hash>,
    signature: Option<Signature>,
    address: Address,
    approver: Address,
    source: Hash,
    stamp: Stamp
}