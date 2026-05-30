use crate::error::Result;
use std::fs;
use std::path::Path;

/// Checks if a file exists at the relative path
pub fn file_exists(relative_path: &str) -> Result<bool> {
    Ok(Path::new(relative_path).exists())
}

/// Creates a file with the given content
pub fn create_boiled_crab_file(relative_path: &str, content: &str) -> Result<()> {
    let full_path = Path::new(relative_path);

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            crate::error::CliError::IoError(e)
        })?;
    }

    fs::write(full_path, content).map_err(|e| {
        crate::error::CliError::IoError(e)
    })?;

    Ok(())
}

/// Updates a module file by adding a module declaration
pub fn update_module_file(path: &str, module_decl: &str) -> Result<()> {
    let mut content = fs::read_to_string(path).map_err(|e| crate::error::CliError::IoError(e))?;
    
    if !content.contains(module_decl) {
        if let Some(pos) = content.rfind("pub mod") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", module_decl));
            }
        } else if let Some(pos) = content.rfind("mod ") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", module_decl));
            }
        } else {
            content.insert_str(0, &format!("{}\n\n", module_decl));
        }
    }
    
    fs::write(path, content).map_err(|e| crate::error::CliError::IoError(e))?;
    Ok(())
}
