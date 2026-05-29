use crate::error::Result;
use dialoguer::Select;
use indicatif::ProgressBar;
use std::time::Duration;

/// Display a spinner-based loading animation
pub struct LoadingSpinner {
    progress_bar: ProgressBar,
}

impl LoadingSpinner {
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_strings(&["|", "/", "-", "\\"])
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));

        Self { progress_bar: pb }
    }

    pub fn finish(&self) {
        self.progress_bar.finish_and_clear();
    }
}

impl Drop for LoadingSpinner {
    fn drop(&mut self) {
        self.progress_bar.finish_and_clear();
    }
}

/// Display a selection menu and return the selected index
pub fn show_menu(prompt: &str, items: &[String]) -> Result<usize> {
    Select::new()
        .with_prompt(prompt)
        .items(items)
        .interact()
        .map_err(|_| crate::error::CliError::InvalidSelection)
}

/// Print success message
pub fn print_success(message: &str) {
    println!("[SUCCESS] {}", message);
}

/// Print info message
pub fn print_info(message: &str) {
    println!("[INFO] {}", message);
}

/// Print error message
pub fn print_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

/// Print warning message
pub fn print_warning(message: &str) {
    eprintln!("[WARN] {}", message);
}
