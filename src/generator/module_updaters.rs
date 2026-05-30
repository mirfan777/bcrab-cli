use crate::error::Result;
use std::fs;
use super::file_ops::update_module_file;
use super::utils::to_title_case;

pub fn update_application_dtos_mod(name: &str) -> Result<()> {
    let title = to_title_case(name);
    let mod_path = "src/application/dtos.rs";
    
    // Add module declaration
    update_module_file(mod_path, &format!("pub mod {};", name))?;
    
    // Add re-export of the types
    let mut content = fs::read_to_string(mod_path).map_err(|e| crate::error::CliError::IoError(e))?;
    let re_export = format!("pub use {}::{{{}Request, {}Response}};", name, title, title);
    
    if !content.contains(&re_export) {
        // Find the last pub use statement and insert after it
        if let Some(pos) = content.rfind("pub use") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", re_export));
            }
        } else {
            // If no pub use exists, add before any existing content
            content.push_str(&format!("\n{}\n", re_export));
        }
    }
    
    fs::write(mod_path, content).map_err(|e| crate::error::CliError::IoError(e))?;
    Ok(())
}

pub fn update_application_services_mod(name: &str) -> Result<()> {
    let title = to_title_case(name);
    let mod_path = "src/application/services.rs";
    
    // Add module declaration
    update_module_file(mod_path, &format!("mod {}_service;", name))?;
    
    // Add re-export of the service
    let mut content = fs::read_to_string(mod_path).map_err(|e| crate::error::CliError::IoError(e))?;
    let re_export = format!("pub use {}_service::{title}Service;", name);
    
    if !content.contains(&re_export) {
        // Find the last pub use statement and insert after it
        if let Some(pos) = content.rfind("pub use") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", re_export));
            }
        } else {
            // If no pub use exists, add after mod declarations
            if let Some(pos) = content.rfind("mod ") {
                if let Some(newline_pos) = content[pos..].find('\n') {
                    let insert_pos = pos + newline_pos + 1;
                    content.insert_str(insert_pos, &format!("\n{}\n", re_export));
                }
            } else {
                content.push_str(&format!("\n{}\n", re_export));
            }
        }
    }
    
    fs::write(mod_path, content).map_err(|e| crate::error::CliError::IoError(e))?;
    Ok(())
}

pub fn update_domain_entities_mod(name: &str) -> Result<()> {
    let title = to_title_case(name);
    let mod_path = "src/domain/entities.rs";
    
    // Add module declaration
    update_module_file(mod_path, &format!("pub mod {};", name))?;
    
    // Add re-export of the entity
    let mut content = fs::read_to_string(mod_path).map_err(|e| crate::error::CliError::IoError(e))?;
    let re_export = format!("pub use {}::{title};", name);
    
    if !content.contains(&re_export) {
        // Find the last pub use statement and insert after it
        if let Some(pos) = content.rfind("pub use") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", re_export));
            }
        } else {
            // If no pub use exists, add after mod declarations
            if let Some(pos) = content.rfind("pub mod") {
                if let Some(newline_pos) = content[pos..].find('\n') {
                    let insert_pos = pos + newline_pos + 1;
                    content.insert_str(insert_pos, &format!("{}\n", re_export));
                }
            } else {
                content.push_str(&format!("\n{}\n", re_export));
            }
        }
    }
    
    fs::write(mod_path, content).map_err(|e| crate::error::CliError::IoError(e))?;
    Ok(())
}

pub fn update_domain_repositories_mod(name: &str) -> Result<()> {
    let title = to_title_case(name);
    let mod_path = "src/domain/repositories.rs";
    
    // Add module declaration
    update_module_file(mod_path, &format!("pub mod {}_repository;", name))?;
    
    // Add re-export of the repository trait
    let mut content = fs::read_to_string(mod_path).map_err(|e| crate::error::CliError::IoError(e))?;
    let re_export = format!("pub use {}_repository::{title}Repository;", name);
    
    if !content.contains(&re_export) {
        // Find the last pub use statement and insert after it
        if let Some(pos) = content.rfind("pub use") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", re_export));
            }
        } else {
            // If no pub use exists, add after mod declarations
            if let Some(pos) = content.rfind("pub mod") {
                if let Some(newline_pos) = content[pos..].find('\n') {
                    let insert_pos = pos + newline_pos + 1;
                    content.insert_str(insert_pos, &format!("{}\n", re_export));
                }
            } else {
                content.push_str(&format!("\n{}\n", re_export));
            }
        }
    }
    
    fs::write(mod_path, content).map_err(|e| crate::error::CliError::IoError(e))?;
    Ok(())
}

pub fn update_infrastructure_database_mod(name: &str) -> Result<()> {
    // Update model.rs file using no-mod.rs pattern
    update_no_mod_file(
        "src/infrastructure/database/model.rs",
        name,
        &format!("{}Model", to_title_case(name)),
        true,
    )?;
    
    // Update repository.rs file using no-mod.rs pattern
    update_no_mod_file(
        "src/infrastructure/database/repository.rs",
        name,
        &format!("{}RepositoryImpl", to_title_case(name)),
        false,
    )?;
    
    Ok(())
}

pub fn update_presentation_handlers_mod(name: &str) -> Result<()> {
    let mod_path = "src/presentation/handlers.rs";
    update_module_file(mod_path, &format!("pub mod {}_handlers;", name))
}

fn update_no_mod_file(path: &str, module_name: &str, export_name: &str, is_model: bool) -> Result<()> {
    let mut content = if std::path::Path::new(path).exists() {
        fs::read_to_string(path).map_err(|e| crate::error::CliError::IoError(e))?
    } else {
        // Create new file with initial structure
        if is_model {
            "pub mod user;\npub mod order;\n\npub use user::Model as UserModel;\npub use order::Model as OrderModel;\n".to_string()
        } else {
            "pub mod user;\npub mod order;\n\npub use user::SeaOrmUserRepository;\npub use order::OrderRepositoryImpl;\n".to_string()
        }
    };

    // Check if module declaration already exists
    let module_decl = format!("pub mod {};", module_name);
    if !content.contains(&module_decl) {
        // Find where to insert the module declaration (after other pub mod lines)
        if let Some(pos) = content.rfind("pub mod") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", module_decl));
            }
        } else {
            content.insert_str(0, &format!("{}\n", module_decl));
        }
    }

    // Check if re-export already exists
    let re_export = if is_model {
        format!("pub use {}::Model as {};", module_name, export_name)
    } else {
        format!("pub use {}::{};", module_name, export_name)
    };
    
    if !content.contains(&re_export) {
        // Find where to insert the re-export (after other pub use lines)
        if let Some(pos) = content.rfind("pub use") {
            if let Some(newline_pos) = content[pos..].find('\n') {
                let insert_pos = pos + newline_pos + 1;
                content.insert_str(insert_pos, &format!("{}\n", re_export));
            }
        } else {
            content.push_str(&format!("\n{}\n", re_export));
        }
    }

    fs::write(path, content).map_err(|e| crate::error::CliError::IoError(e))?;
    Ok(())
}
