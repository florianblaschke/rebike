use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;

pub fn rebike_personio() -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let xml = reqwest::blocking::get("https://rebike-mobility.jobs.personio.de/xml")?.text()?;
    let jobs = read_steam(&xml)?;

    Ok(jobs)
}

pub fn read_steam(
    xml_feed: &str,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml_feed);
    reader.config_mut().trim_text(true);

    let mut count = 0;
    let mut path_parts = Vec::new();
    let mut map = HashMap::new();
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
                    // Count the jobs
                    count += 1;
                    // position signals a new job, so push the last map into an array
                    if !map.is_empty() {
                        jobs.push(map);
                        // create a new hash for the next jop
                        map = HashMap::new();
                    }
                    continue;
                }
                let elem_name = String::from_utf8(e.name().as_ref().to_owned())?;
                path_parts.push(elem_name);
            }
            Ok(Event::CData(e)) => {
                let text = e.decode()?.trim().to_string();
                if !text.is_empty() && !path_parts.is_empty() {
                    let full_path = path_parts.join(".");
                    map.insert(full_path, text);
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.decode()?.to_string();
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

        let jobs = read_steam(txt).unwrap();
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

        let jobs = read_steam(txt).unwrap();
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

        let jobs = read_steam(txt).unwrap();
        let map = jobs[0].clone();

        assert_eq!(
            map.get("name").unwrap().to_owned(),
            "The name is John Cena.".to_string()
        );
    }
}
