#[derive(Debug, PartialEq, Eq)]
            .map(|p| p.to_string_lossy().into_owned())
            .map(|p| p.to_string_lossy().into_owned())
        let name = get_repeated_file_path_from_diff_line(&self.diff_line).unwrap_or_default();
            let name = get_repeated_file_path_from_diff_line(&self.diff_line).unwrap_or_default();
fn remove_surrounding_quotes(path: &str) -> &str {
    if path.starts_with('"') && path.ends_with('"') {
        // Indexing into the UTF-8 string is safe because of the previous test
        &path[1..path.len() - 1]
    } else {
        path
    }
}

    let path = match s.strip_suffix('\t').unwrap_or(s) {
    };
    // When a path contains non-ASCII characters, a backslash, or a quote then it is quoted,
    // so remove these quotes. Characters may also be escaped, but these are left as-is.
    remove_surrounding_quotes(path).to_string()

        assert_eq!(
            parse_diff_header_line("+++ \".\\delta.rs\"", true),
            (".\\delta.rs".to_string(), FileEvent::Change)
        );