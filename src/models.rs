/// Represents a code template available for cloning
#[derive(Debug, Clone)]
pub struct Template {
    pub id: u8,
    pub name: String,
    pub url: String,
    pub description: String,
    pub branches: Option<Vec<String>>,
    pub configurations: Option<Vec<String>>,
}

impl Template {
    pub fn new(id: u8, name: &str, url: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            url: url.to_string(),
            description: description.to_string(),
            branches: None,
            configurations: None,
        }
    }

    pub fn with_branches(mut self, branches: Vec<&str>) -> Self {
        self.branches = Some(branches.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn with_configurations(mut self, configurations: Vec<&str>) -> Self {
        self.configurations = Some(configurations.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn display_text(&self) -> String {
        format!("{} - {}", self.name, self.description)
    }
}

/// Represents a clone operation configuration
#[derive(Debug, Clone)]
pub struct CloneConfig {
    pub template: Template,
    pub app_name: String,
    pub branch: Option<String>,
}

impl CloneConfig {
    pub fn new(template: Template, app_name: String) -> Self {
        Self {
            template,
            app_name,
            branch: None,
        }
    }

    pub fn with_branch(mut self, branch: String) -> Self {
        self.branch = Some(branch);
        self
    }
}
