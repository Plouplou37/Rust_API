use {
    crate::miner::*,
    serde::{Deserialize, Serialize},
};

// ------------ JSON Payload (REST)

#[derive(Debug, Deserialize, Serialize)] //Automatically generate implementations for the traits Debug, Deserialize, and Serialize
pub struct Wallet {
    // Attributes of the struct Miner
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32,
    pub total_shares_mined: i32, // MH.s-1
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

// ------------ POST Request Body for New Wallet

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    pub clubname: String,
}

// ------------ DAO(Data Access Object) --> DB Table Records
//Insert and Select Records from the DB
pub struct WalletDAO {
    pub address: String,
    pub club_name: String,
}
