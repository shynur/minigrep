pub fn search<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
    let mut dest_lines = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            dest_lines.push(line);
        }
    }
    dest_lines
}

pub fn search_case_insensitive<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
    let query = query.to_lowercase();
    let mut dest_lines = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query[..]) {
            dest_lines.push(line);
        }
    }
    dest_lines
}
