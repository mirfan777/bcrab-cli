use std::path::Path;

/// Converts a string to title case (first letter uppercase)
pub fn to_title_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Checks if CRAB file exists in the current directory
pub fn crab_file_exists() -> bool {
    Path::new("CRAB").exists()
}
