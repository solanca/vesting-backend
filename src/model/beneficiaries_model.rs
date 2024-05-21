use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Beneficiaries {
    pub _id:ObjectId,
    pub index:u32,
    pub key:String,
    #[serde(rename = "allocatedTokens")]
    pub allocated_tokens:u64,
    #[serde(rename = "claimedTokens")]
    pub claimed_tokens:u64,
    #[serde(rename = "lastClaimTime")]
    pub last_claim_time:u64
}