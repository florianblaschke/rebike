use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::Serialize;
use std::collections::HashMap;

use crate::job::slug::format_slug;

use super::html::{filter_out_tags, get_li_items};

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
/// Todo: Implement multiple languages, test with edri feed
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

// #[derive(Debug, Serialize)]
// enum LocationKind {
//     OnSite,
//     Remote,
//     Hybrid,
// }

// impl TryFrom<&str> for LocationKind {
//     type Error = Box<dyn std::error::Error>;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let kind = match value {
//             "office" => Self::OnSite,
//             "store" => Self::OnSite,
//             "recruitin"
//             _ => return Err(format!("invalid locationkind found: {}", value).into()),
//         };

//         Ok(kind)
//     }
// }

#[derive(Debug, Serialize)]
struct Facet {
    key: String,
    values: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    tenant_id: ObjectId,
    status: String,
    published_at: DateTime,
    title: MultiLanguageObj<LanguageDetailsString>,
    description: MultiLanguageObj<LanguageDetailsString>,
    locations: Vec<Location>,
    facets: Vec<Facet>,
    version: u32,
    is_deleted: bool,
    created_at: DateTime,
    created_by: String,
    public_id: String,
    slug: MultiLanguageObj<LanguageDetailsString>,
    updated_at: DateTime,
    updated_by: String,
    benefits: MultiLanguageObj<LanguageDetailsVec>,
    requirements: MultiLanguageObj<LanguageDetailsVec>,
    responsibilities: MultiLanguageObj<LanguageDetailsVec>,
    recruiter_ids: Option<Vec<String>>,
    benefit_group_id: ObjectId,
}

impl TryFrom<&HashMap<String, String>> for Job {
    type Error = Box<dyn std::error::Error>;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Box<dyn std::error::Error>> {
        let description =
            filter_out_tags(&get_value("jobDescriptions.jobDescription.value", map)?)?;

        let responsibilities =
            get_li_items(&get_value("jobDescriptions.jobDescription.1.value", map)?)?;

        let requirements =
            get_li_items(&get_value("jobDescriptions.jobDescription.2.value", map)?)?;

        let benefits = get_li_items(&get_value("jobDescriptions.jobDescription.3.value", map)?)?;
        let job = Job {
            //hardcode tenant_id
            tenant_id: ObjectId::parse_str("695245d5023deb54ecbf0f8d")?,
            // tenant_id: ObjectId::parse_str("6911eefc837972cf2d3f68e5")?,
            status: "published".to_string(),
            published_at: DateTime::now(),
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
                country: "DE".to_string(),
                kind: "onsite".to_string(),
            }],
            facets: vec![
                Facet {
                    key: "employmentTypes".to_string(),
                    values: vec!["full_time".to_string()],
                },
                Facet {
                    key: "experienceLevels".to_string(),
                    values: vec!["experienced".to_string()],
                },
                Facet {
                    key: "workingModel".to_string(),
                    values: vec!["on_site".to_string()],
                },
            ],
            version: 1,
            is_deleted: false,
            created_at: DateTime::parse_rfc3339_str(get_value("createdAt", map)?)?,
            public_id: cuid::cuid2_slug(),
            slug: MultiLanguageObj {
                de: LanguageDetailsString {
                    custom: Some(format_slug(&get_value("name", map)?)?),
                    default: None,
                },
            },
            updated_at: DateTime::now(),
            updated_by: "rebike-importer".to_string(),
            created_by: "rebike-importer".to_string(),
            benefits: MultiLanguageObj {
                de: LanguageDetailsVec {
                    custom: Some(clean_list_items(benefits)?),
                    default: None,
                },
            },
            requirements: MultiLanguageObj {
                de: LanguageDetailsVec {
                    custom: Some(clean_list_items(requirements)?),
                    default: None,
                },
            },
            responsibilities: MultiLanguageObj {
                de: LanguageDetailsVec {
                    custom: Some(clean_list_items(responsibilities)?),
                    default: None,
                },
            },
            recruiter_ids: None,
            benefit_group_id: ObjectId::parse_str("695bb697eaa9de0d07ba7f70")?,
        };

        Ok(job)
    }
}

fn clean_list_items(items: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut cleaned_items = Vec::new();

    for item in items {
        let cleaned = filter_out_tags(&item)?;
        cleaned_items.push(cleaned);
    }

    Ok(cleaned_items)
}

fn get_value(key: &str, map: &HashMap<String, String>) -> Result<String, String> {
    let value = match map.get(key) {
        Some(v) => v.to_owned(),
        None => "".to_string(),
    };

    Ok(value)
}
