use importer_rebike::{config::get_configuration, db, job::def::Job, xml::xml_feed};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration().expect("unable to read configuration");
    let jobs = xml_feed(&config.ats.endpoint)?;

    let mut all_jobs = Vec::new();
    for job in jobs {
        let j = Job::try_from(&job)?;
        all_jobs.push(j);
    }

    let collection = db::job_collection(&config.db.uri)?;
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
