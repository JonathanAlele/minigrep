pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "with";
        let contents = "\
So much depends upon a red wheelbarrow
glazed with rainwater
beside the white chickens
        ";
        assert_eq!(
            vec!["glazed with rainwater"],
            search_case_sensitive(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "Glazed";
        let contents = "\
So much depends upon a red wheelbarrow
Glazed with rainwater
Beside the white chickens";
        assert_eq!(
            vec!["Glazed with rainwater"],
            search_case_insensitive(query, contents)
        );
    }
}
