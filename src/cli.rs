use crate::error::Result;
use crate::models::CloneConfig;
use crate::repository::clone_repository;
use crate::templates::{get_template_by_id, get_template_display_options};
use crate::ui::{print_info, show_menu};

/// Handle the create new command
pub fn handle_create_new(app_name: String) -> Result<()> {
    print_info(&format!("Creating new app: {}\n", app_name));

    // Step 1: Show template selection
    let templates_options = get_template_display_options();
    let selected_template_id = show_menu("Select template to use", &templates_options)?;

    let template = get_template_by_id(selected_template_id as u8)
        .ok_or(crate::error::CliError::InvalidSelection)?;

    let mut config = CloneConfig::new(template.clone(), app_name);

    // Step 2: If template has branches, show branch selection
    if let Some(branches) = &template.branches {
        let branch_options: Vec<String> = branches.iter().map(|b| b.to_string()).collect();
        let selected_branch = show_menu(&format!("Select branch for {}", template.name), &branch_options)?;
        let branch_name = branches[selected_branch].clone();

        println!();
        print_info(&format!(
            "Cloning {} ({}) as '{}'\n",
            template.name, branch_name, config.app_name
        ));

        config = config.with_branch(branch_name);
    }
    // Step 3: If template has configurations (but no branches), show configuration selection
    else if let Some(configs) = &template.configurations {
        let config_options: Vec<String> = configs.iter().map(|c| c.to_string()).collect();
        let selected_config = show_menu(&format!("Select configuration for {}", template.name), &config_options)?;
        let _config_name = configs[selected_config].clone();

        println!();
        print_info(&format!(
            "Cloning {} (config: {}) as '{}'\n",
            template.name, _config_name, config.app_name
        ));

        // Configuration selected but not used for cloning yet (placeholder)
    } else {
        println!();
        print_info(&format!(
            "Cloning {} repository as '{}'\n",
            template.name, config.app_name
        ));
    }

    // Step 4: Clone repository
    clone_repository(&config)?;

    Ok(())
}
