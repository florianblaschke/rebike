mod connect;
mod html;
mod job;
mod read;

use std::fs;

use job::Job;
use read::rebike_personio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jobs = rebike_personio()?;

    let mut all_jobs = Vec::new();
    for job in jobs {
        let j = Job::try_from(&job)?;
        all_jobs.push(j);
    }

    let _ = fs::write(
        "src/output/jobs.json",
        serde_json::to_string_pretty(&all_jobs)?,
    );

    Ok(())
}
