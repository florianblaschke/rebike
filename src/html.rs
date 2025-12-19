use regex::Regex;

pub fn get_li_items(html_string: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let regex = Regex::new(r"<li>(.*?)</li>").unwrap();
    let mut items = Vec::new();
    for (_, [item]) in regex.captures_iter(html_string).map(|c| c.extract()) {
        items.push(item.trim().to_owned());
    }
    Ok(items)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_li() {
        let html_string = r#"<li>First Item</li>"#;
        let result = get_li_items(&html_string).unwrap();
        let mut iterator = result.iter();

        assert_eq!(iterator.next(), Some(&"First Item".to_string()));
    }

    #[test]
    fn extract_multiple_li() {
        let html_string = r#"<li>First Item</li><li>Second Item</li><li>Third Item</li>"#;
        let result = get_li_items(&html_string).unwrap();
        let mut iterator = result.iter();

        assert_eq!(iterator.next(), Some(&"First Item".to_string()));
        assert_eq!(iterator.next(), Some(&"Second Item".to_string()));
        assert_eq!(iterator.next(), Some(&"Third Item".to_string()));
    }
}
