use mongodb::{bson::doc, error::Error, Client, Collection};

use crate::model::beneficiaries_model::Beneficiaries;
#[derive(Clone)]
pub struct Database {
    pub beneficiaries:Collection<Beneficiaries>
}

impl Database {
    pub async fn _init() -> Self {
        let url = "mongodb+srv://zhongxi1992:1FIZfgsoYDkS0Bg3@cluster0.x56nkq9.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string();
      let client =   Client::with_uri_str(url.clone()).await.unwrap();
       let db =client.database("test");
       let beneficiaries = db.collection("beneficiaries");
       Database {
        beneficiaries
    }} 

    pub async fn update_beneficiary(&self,address:String) -> Result<Option<Beneficiaries>,Error>{

        let result = self.beneficiaries.find_one_and_update(doc! {"key":address}, doc!{"$set":{"claimedTokens":1000,"lastClaimTime":1000000}}, None).await.ok().expect("failed the ");
        Ok(result)
    }
}