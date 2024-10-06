use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub root_dir: String,
    pub build_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub repository: String,
    pub branch: String,
    pub token: String,
    pub pull_dir: String,
    pub services: Vec<Service>,
}

impl Default for Service {
    fn default() -> Self {
        Service {
            name: "service-name".to_owned(),
            root_dir: "/var/www/your_repository/backend/my_service".to_owned(),
            build_dir: "/var/www/my_service".to_owned(),
        }
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            branch: "main".to_owned(),
            repository: "github.com/your-repository/link".to_owned(),
            token: "YOUR-GITHUB-TOKEN-HERE".to_owned(),
            pull_dir: "/var/www".to_owned(),
            services: vec![Service::default()],
        }
    }
}
