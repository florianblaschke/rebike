use regex::Regex;

const UNALLOWED_CHARCATERS: [&str; 7] = ["ä", "ö", "ü", "(", ")", "/", " "];

pub fn format_slug(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let regex = Regex::new(r"(-{2,})").unwrap();
    let lowercased = input.to_lowercase();
    let whitespace_removed = lowercased.trim().replace(" ", "-");
    let segments = whitespace_removed.split("");

    let mut slug = String::new();
    for char in segments {
        if UNALLOWED_CHARCATERS.contains(&char) {
            slug.push_str("|");
            continue;
        }
        slug.push_str(char);
    }
    let stripped_replacement = slug.replace("|", "");
    let cleaned = regex.replace_all(&stripped_replacement, "-");
    Ok(cleaned.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    struct Test {
        input: String,
        expected: String,
    }

    #[test]
    fn test_multiple_whitespace() {
        let tests = vec![
            Test {
                input: "Fahrradmechaniker (E-) Bike-Werkstatt - mit Wechselprämie (m/w/d)"
                    .to_string(),
                expected: "fahrradmechaniker-e-bike-werkstatt-mit-wechselprmie-mwd".to_string(),
            },
            Test {
                input: "Fahrradmechaniker im Kundenservice / Remote (m/w/d)".to_string(),
                expected: "fahrradmechaniker-im-kundenservice-remote-mwd".to_string(),
            },
            Test {
                input: "Fahrradmonteur Azubi Sommer 2026 (m/w/d)".to_string(),
                expected: "fahrradmonteur-azubi-sommer-2026-mwd".to_string(),
            },
        ];

        for test in tests {
            let result = format_slug(&test.input).unwrap();
            assert_eq!(result, test.expected)
        }
    }
}
