use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Beneficiaries {
    pub _id: ObjectId,
    pub wallet: String,
    pub total: f64,
    pub farm: f64,
    pub yetiz: f64,
    pub presale: f64,
    pub pixiz: f64,
    #[serde(rename = "claimedTokens")]
    pub claimed_tokens: f64,
    #[serde(rename = "lastClaimTime")]
    pub last_claim_time: i64,
    // pub proof:Vec<String>
}

#[derive(Deserialize)]
pub struct SubmitTransactionRequest {
    pub transaction: String,
}
