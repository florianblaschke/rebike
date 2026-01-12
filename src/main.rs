use importer_rebike::{config::get_configuration, db, job::def::Job, xml::read};
use mongodb::bson::doc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration().expect("unable to read configuration");
    let jobs = read(&config.ats.endpoint)?;

    let mut all_jobs = Vec::new();
    for job in jobs {
        match Job::try_from(&job) {
            Ok(j) => {
                all_jobs.push(j);
            }
            Err(s) => println!("could not create job: {}", s),
        };
    }

    let collection = db::job_collection(&config.db.uri)?;
    collection
        .delete_many(doc! { "createdBy": "rebike-importer" })
        .run()?;
    let result = collection.insert_many(all_jobs).run()?;
    for id in result.inserted_ids {
        println!("inserted job with id: {:?}", id.1);
    }
    Ok(())
}
