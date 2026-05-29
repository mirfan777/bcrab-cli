use crate::models::Template;

/// Get all available templates
pub fn get_templates() -> Vec<Template> {
    vec![
        Template::new(
            2,
            "boiled-crab-svelte",
            "https://github.com/mirfan777/boiled-crab-svelte",
            "Svelte Frontend",
        )
        .with_branches(vec!["SPA", "SSR"]),
        Template::new(
            3,
            "boiled-crab",
            "https://github.com/mirfan777/boiled-crab",
            "API Server",
        )
        .with_configurations(vec!["basic", "teams"]),
    ]
}

/// Get a template by ID
pub fn get_template_by_id(id: u8) -> Option<Template> {
    get_templates().into_iter().find(|t| t.id == id)
}

/// Get display options for template selection
pub fn get_template_display_options() -> Vec<String> {
    get_templates()
        .iter()
        .map(|t| t.display_text())
        .collect()
}
