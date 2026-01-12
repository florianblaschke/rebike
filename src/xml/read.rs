use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;

pub fn xml_feed(url: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let xml = reqwest::blocking::get(url)?.text()?;
    let jobs = read_stream(&xml)?;

    Ok(jobs)
}

pub fn read_stream(
    xml_feed: &str,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml_feed);
    reader.config_mut().trim_text(true);

    let mut count = 0;
    let mut path_parts = Vec::new();
    let mut map = HashMap::new();
    let mut element_counts = HashMap::<String, u32>::new();
    let mut jobs = Vec::new();

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"workzag-jobs" {
                    continue;
                }
                if e.name().as_ref() == b"position" {
                    count += 1;
                    if !map.is_empty() {
                        jobs.push(map);
                        map = HashMap::new();
                    }
                    path_parts.clear();
                    element_counts.clear();
                    continue;
                }

                let elem_name = String::from_utf8(e.name().as_ref().to_owned())?;

                let parent_path = path_parts.join(".");
                let context_key = if parent_path.is_empty() {
                    elem_name.clone()
                } else {
                    format!("{}.{}", parent_path, elem_name)
                };

                let current_count = *element_counts.get(&context_key).unwrap_or(&0);
                element_counts.insert(context_key, current_count + 1);

                if current_count > 0 {
                    path_parts.push(format!("{}.{}", elem_name, current_count));
                } else {
                    path_parts.push(elem_name);
                }
            }
            Ok(Event::CData(e)) => {
                let text = e.decode()?.trim().to_string();
                if !text.is_empty() && !path_parts.is_empty() {
                    let full_path = path_parts.join(".");
                    map.insert(full_path, text);
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.decode()?.trim().to_string();
                if !text.is_empty() && !path_parts.is_empty() {
                    let full_path = path_parts.join(".");
                    map.insert(full_path, text);
                }
            }
            Ok(Event::End(_)) => {
                path_parts.pop();
            }
            // Do not listen to other events for now
            _ => (),
        }
    }

    // Push data if it was just a single job
    if !map.is_empty() {
        jobs.push(map);
    }

    println!("{} jobs read", count);

    Ok(jobs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_feed() {
        let txt = r#"<position>
                        <name>IT</name>
                    </position>"#;

        let jobs = read_stream(txt).unwrap();
        let map = jobs[0].clone();

        assert_eq!(map.get("name").unwrap().to_owned(), "IT".to_string())
    }

    #[test]
    fn test_read_nested_feed() {
        let txt = r#"<position>
                        <name>IT</name>
                        <location>
                            <city>Somewhere</city>
                            <state>Somestate</state>
                        </location>
                    </position>"#;

        let jobs = read_stream(txt).unwrap();
        let map = jobs[0].clone();

        assert_eq!(
            map.get("location.city").unwrap().to_owned(),
            "Somewhere".to_string()
        );
        assert_eq!(
            map.get("location.state").unwrap().to_owned(),
            "Somestate".to_string()
        );
    }

    #[test]
    fn test_read_cdata() {
        let txt = r#"<position>
                        <name><![CDATA[ The name is John Cena. ]]></name>
                    </position>"#;

        let jobs = read_stream(txt).unwrap();
        let map = jobs[0].clone();

        assert_eq!(
            map.get("name").unwrap().to_owned(),
            "The name is John Cena.".to_string()
        );
    }

    #[test]
    fn test_read_double_keys() {
        let txt = r#"<position>
                        <jobDescriptions>
                            <jobDescription>
                                <name>First</name>
                                <value>First Value</value>
                            </jobDescription>
                            <jobDescription>
                                <name>Second</name>
                                <value>Second Value</value>
                            </jobDescription>
                        </jobDescriptions>
                    </position>"#;

        let jobs = read_stream(txt).unwrap();
        let map = jobs[0].clone();

        assert_eq!(
            map.get("jobDescriptions.jobDescription.name")
                .unwrap()
                .to_owned(),
            "First".to_string()
        );
        assert_eq!(
            map.get("jobDescriptions.jobDescription.1.name")
                .unwrap()
                .to_owned(),
            "Second".to_string()
        );
    }
}
