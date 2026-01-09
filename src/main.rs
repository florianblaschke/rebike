use importer_rebike::{db, job::def::Job, xml::rebike_personio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let jobs = rebike_personio()?;

    let mut all_jobs = Vec::new();
    for job in jobs {
        let j = Job::try_from(&job)?;
        all_jobs.push(j);
    }

    let collection = db::job_collection()?;
    let result = collection.insert_many(all_jobs).run()?;
    println!("Inserted documents with _ids:");
    for (_key, value) in &result.inserted_ids {
        println!("{}", value);
    }
    // let _ = fs::write(
    //     "src/output/jobs.json",
    //     serde_json::to_string_pretty(&all_jobs)?,
    // );

    Ok(())
}
