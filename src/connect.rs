use mongodb::{
    bson::Document,
    sync::{Client, Collection},
};

pub fn _job_collection() -> Result<Collection<Document>, Box<dyn std::error::Error>> {
    let monogo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");

    let client = Client::with_uri_str(monogo_uri)?;
    let db = client.database("career_stage");
    let job_collection: Collection<Document> = db.collection("jobs");

    Ok(job_collection)
}
