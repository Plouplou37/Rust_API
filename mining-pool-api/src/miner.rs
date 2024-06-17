use serde::{Deserialize, Serialize};

// ------------ JSON Payload (REST)

#[derive(Debug, Deserialize, Serialize)] //Automatically generate implementations for the traits Debug, Deserialize, and Serialize
pub struct Miner {
    // Attributes of the struct Miner
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32, // MH.s-1
    pub shared_mined: i32,
}

// ------------ POST Request Body for New Miner

#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
    pub nickname: String,
}

// ------------ DAO(Data Access Object) --> DB Table Records
//Insert and Select Records from the DB
pub struct MinerDAO {
    pub id: String,
    pub address: String,
    pub nickname: String,
    pub hash_rate: i32, // MH.s-1
    pub shared_mined: i32,
}
