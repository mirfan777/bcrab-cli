use crate::error::Result;
use crate::ui::{print_info, print_success, LoadingSpinner};
use super::file_creators::*;
use super::utils::to_title_case;

pub fn handle_make_resources(name: String) -> Result<()> {
    print_info(&format!("Generating resource: {}\n", &name));

    let spinner = LoadingSpinner::new(&format!("Creating {} resource scaffold...", name));
    
    // Convert name to appropriate cases
    let singular_lower = name.to_lowercase();
    let singular_title = to_title_case(&name);
    
    // Create DTOs
    create_dto_file(&singular_lower, &singular_title)?;
    
    // Create Service
    create_service_file(&singular_lower, &singular_title)?;
    
    // Create Entity
    create_entity_file(&singular_lower, &singular_title)?;
    
    // Create Repository trait and implementation
    create_repository_trait_file(&singular_lower, &singular_title)?;
    create_repository_impl_file(&singular_lower, &singular_title)?;
    
    // Create Database Model
    create_database_model_file(&singular_lower, &singular_title)?;
    
    // Create Handler
    create_handler_file(&singular_lower, &singular_title)?;
    
    spinner.finish();
    
    print_success(&format!(
        "Resource '{}' generated successfully!\n\nGenerated files:\n  - src/application/dtos/{}.rs\n  - src/application/services/{}_service.rs\n  - src/domain/entities/{}.rs\n  - src/domain/repositories/{}_repository.rs\n  - src/infrastructure/database/model/{}.rs\n  - src/infrastructure/database/repository/{}.rs\n  - src/presentation/handlers/{}_handlers.rs",
        singular_lower, singular_lower, singular_lower, singular_lower, singular_lower, singular_lower, singular_lower, singular_lower
    ));
    
    print_info("Next steps:\n  1. Update field definitions in generated files\n  2. Implement service and repository methods\n  3. Add proper error handling\n  4. Register handlers in your router\n");
    
    Ok(())
}

pub fn handle_make_dto(name: String) -> Result<()> {
    print_info(&format!("Generating DTO: {}\n", &name));

    let spinner = LoadingSpinner::new(&format!("Creating {} DTO...", name));
    
    let singular_lower = name.to_lowercase();
    let singular_title = to_title_case(&name);
    
    // Create DTOs
    create_dto_file(&singular_lower, &singular_title)?;
    
    spinner.finish();
    
    print_success(&format!(
        "DTO '{}' generated successfully!\n\nGenerated file:\n  - src/application/dtos/{}.rs",
        singular_lower, singular_lower
    ));
    
    print_info("Update the generated DTO file with your fields and implement as needed.\n");
    
    Ok(())
}
