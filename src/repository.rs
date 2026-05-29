use crate::error::{CliError, Result};
use crate::models::CloneConfig;
use crate::ui::{print_info, print_success, print_warning, LoadingSpinner};
use std::fs;
use std::process::Command;

/// Clone a repository based on the configuration
pub fn clone_repository(config: &CloneConfig) -> Result<()> {
    let _spinner = LoadingSpinner::new("Cloning repository...");

    let mut cmd = Command::new("git");
    cmd.arg("clone");

    if let Some(branch) = &config.branch {
        cmd.arg("--branch").arg(branch);
    }

    cmd.arg(&config.template.url)
        .arg(&config.app_name);

    let output = cmd
        .output()
        .map_err(|e| CliError::GitError(e.to_string()))?;

    drop(_spinner); // Explicitly drop spinner before printing

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(CliError::CloneError(error_msg));
    }

    // Clean up .git directory
    cleanup_git_directory(&config.app_name)?;

    // Print success message
    print_clone_success_message(config);

    Ok(())
}

/// Remove the .git directory from the cloned project
fn cleanup_git_directory(app_name: &str) -> Result<()> {
    let git_dir = format!("{}/.git", app_name);
    fs::remove_dir_all(&git_dir).map_err(|e| {
        print_warning(&format!("Could not remove .git directory: {}", e));
        CliError::CleanupError(e.to_string())
    })?;
    Ok(())
}

/// Print success message for clone operation
fn print_clone_success_message(config: &CloneConfig) {
    let message = if let Some(branch) = &config.branch {
        format!(
            "Successfully created project '{}' from {} (branch: {})",
            config.app_name, config.template.name, branch
        )
    } else {
        format!(
            "Successfully created project '{}' from {}",
            config.app_name, config.template.name
        )
    };

    print_success(&message);
    println!();
    print_info("Navigate to your project:");
    print_info(&format!("  cd {}", config.app_name));
}
