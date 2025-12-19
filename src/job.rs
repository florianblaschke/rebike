use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

use crate::html::get_li_items;

#[derive(Debug, Serialize)]
struct LanguageDetailsString {
    custom: Option<String>,
    default: Option<String>,
}

#[derive(Debug, Serialize)]
struct LanguageDetailsVec {
    custom: Option<Vec<String>>,
    default: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct MultiLanguageObj<T> {
    de: T,
}

#[derive(Debug, Serialize)]
struct Location {
    kind: String,
    city: String,
    country: String,
}

#[derive(Debug, Serialize)]
struct Facet {
    key: String,
    values: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Job {
    tenant_id: String,
    status: String,
    published_at: DateTime<Utc>,
    title: MultiLanguageObj<LanguageDetailsString>,
    description: MultiLanguageObj<LanguageDetailsString>,
    locations: Vec<Location>,
    facets: Vec<Facet>,
    version: u32,
    is_deleted: bool,
    created_at: DateTime<Utc>,
    created_by: String,
    public_id: String,
    slug: MultiLanguageObj<LanguageDetailsString>,
    updated_at: DateTime<Utc>,
    updated_by: String,
    benefits: MultiLanguageObj<LanguageDetailsVec>,
    requirements: MultiLanguageObj<LanguageDetailsVec>,
    responsibilities: MultiLanguageObj<LanguageDetailsVec>,
    recruiter_ids: Option<Vec<String>>,
}

impl TryFrom<&HashMap<String, String>> for Job {
    type Error = Box<dyn std::error::Error>;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut description = get_value("jobDescriptions.jobDescription.name", map)?;
        description.push_str(&get_value("jobDescriptions.jobDescription.value", map)?);

        let responsibilities =
            get_li_items(&get_value("jobDescriptions.jobDescription.1.value", map)?)?;

        let requirements =
            get_li_items(&get_value("jobDescriptions.jobDescription.2.value", map)?)?;

        let benefits = get_li_items(&get_value("jobDescriptions.jobDescription.3.value", map)?)?;

        let job = Job {
            //hardcode tenant_id
            tenant_id: "12345".to_string(),
            status: "published".to_string(),
            published_at: Utc::now(),
            title: MultiLanguageObj {
                de: LanguageDetailsString {
                    custom: Some(get_value("name", map)?),
                    default: None,
                },
            },
            description: MultiLanguageObj {
                de: LanguageDetailsString {
                    custom: Some(description),
                    default: None,
                },
            },
            locations: vec![Location {
                city: get_value("office", map)?,
                country: get_value("office", map)?,
                kind: "".to_string(),
            }],
            facets: vec![Facet {
                key: "experienceLevels".to_string(),
                values: vec![get_value("seniority", map)?],
            }],
            version: 1,
            is_deleted: false,
            created_at: Utc::now(),
            public_id: "12345".to_string(),
            slug: MultiLanguageObj {
                de: LanguageDetailsString {
                    custom: Some(get_value("name", map)?.to_lowercase().replace(" ", "-")),
                    default: None,
                },
            },
            updated_at: Utc::now(),
            updated_by: "rebike-importer".to_string(),
            created_by: "rebike-importer".to_string(),
            benefits: MultiLanguageObj {
                de: LanguageDetailsVec {
                    custom: Some(benefits),
                    default: None,
                },
            },
            requirements: MultiLanguageObj {
                de: LanguageDetailsVec {
                    custom: Some(requirements),
                    default: None,
                },
            },
            responsibilities: MultiLanguageObj {
                de: LanguageDetailsVec {
                    custom: Some(responsibilities),
                    default: None,
                },
            },
            recruiter_ids: None,
        };

        Ok(job)
    }
}

fn get_value(key: &str, map: &HashMap<String, String>) -> Result<String, String> {
    let value = match map.get(key) {
        Some(v) => v.to_owned(),
        None => "".to_string(),
    };

    Ok(value)
}

#[cfg(test)]
mod test {
    use crate::read::rebike_personio;

    use super::*;

    #[test]
    fn rebike_create_job() {
        let feed = rebike_personio().unwrap();
        let job = Job::try_from(&feed[0].clone()).unwrap();

        println!("{:?}", job);
    }
}
