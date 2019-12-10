pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
	let mut line_counter = 0;
    for line in content.lines() {
    	line_counter+=1;
        if line.contains(pattern) {
            writeln!(writer, "{}: \t{}", line_counter, line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
