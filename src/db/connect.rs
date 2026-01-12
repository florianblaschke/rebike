use mongodb::sync::{Client, Collection};

use crate::job::def::Job;

pub fn job_collection(monogo_uri: &str) -> Result<Collection<Job>, Box<dyn std::error::Error>> {
    let client = Client::with_uri_str(monogo_uri)?;
    let db = client.database("career_stage");
    let job_collection: Collection<Job> = db.collection("jobs");

    Ok(job_collection)
}
