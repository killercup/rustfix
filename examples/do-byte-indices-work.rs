extern crate serde_json;
extern crate rustfix;

use std::process::Command;

use rustfix::diagnostics::Diagnostic;

fn main() {
    let sorry_for_the_churn = Command::new("touch").arg("src/diagnostics.rs").output();
    let output = Command::new("cargo")
        .args(&["rustc",
                "--lib",
                "--quiet",
                "--color", "never",
                "--",
                "-Z", "unstable-options",
                "--error-format", "json"])
        .output()
        .unwrap();
    let content = String::from_utf8(output.stderr).unwrap();

    let suggestions = content.lines()
        .filter(|s| !s.trim().is_empty())
        .flat_map(|line| serde_json::from_str::<Diagnostic>(line))
        .flat_map(|diag| diag.spans.into_iter())
        .take(5);

    for s in suggestions {
        let file = read_file_to_string(&s.file_name).unwrap();
        let content_by_byte_index = file[s.byte_start..s.byte_end].to_string();
        let content_by_line_col_index: String = file.lines()
            .skip(s.line_start - 1)
            .take(s.line_end - s.line_start + 1)
            .enumerate()
            .map(|(i, line)| {
                let start = 0;
                let end = s.line_end - s.line_start;
                if i == start && i == end {
                    line.chars().skip(s.column_start - 1).take(s.column_end - s.column_start).collect()
                } else if i == start {
                    line.chars().skip(s.column_start - 1).collect()
                } else if i == end {
                    line.chars().take(s.column_end).collect()
                } else {
                    line.to_string()
                }
            })
            .collect();
        assert_eq!(content_by_byte_index, content_by_line_col_index);
        println!("Worked.");
    }
}

fn read_file_to_string(file_name: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_name)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
