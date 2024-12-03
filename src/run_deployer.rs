use crate::generate_conf::file_struct::ConfigFile;
use serde_json;
use std::{fs::File, io::Read, path::Path};

mod pull;

use pull::{ping, RepositoryInfo};

/// Function that starts Deployer. It makes
/// request to GitHub's REST API every 60 seconds.
/// As an argument it takes path to the config file.
///
/// Panics if directory does not exist, no services
/// specified, token/repository/branch is not specified or
/// repository link is invalid.
pub async fn run(path: &str) {
    let config = deserialise(path);
    if config.token.is_empty() || config.token == "YOUR-GITHUB-TOKEN-HERE" {
        panic!("Github token is not specified!");
    }
    if config.repository.is_empty()
        || config.repository == "https://github.com/your-repository/link"
    {
        panic!("Github repository is not specified!");
    }
    let repository = url_fmt(&config.repository, &config.branch);

    // NOTE: Only global directories are valid yet
    validate_dir(&config.pull_dir);
    if config.services.len() == 0 {
        panic!("Not a single service specified :<");
    }

    ping(&config, &repository).await.unwrap();
}

/// Formats URL from `github.com/author/their-repo` to
/// `https://api.github.com/repos/author/their-repo/commits`.
/// Panics if URL is badly formatted.
fn url_fmt<'a>(url: &'a str, branch: &'a str) -> RepositoryInfo<'a> {
    const INVALID_URL: &str = "Invalid repository URL!";

    let list: Vec<&str> = if url.starts_with("https://") {
        url[8..].split('/').collect()
    } else {
        url.split('/').collect()
    };

    if list.len() != 3 {
        panic!("{}", INVALID_URL);
    }

    let domain = list[0];
    let author = list[1];
    let repository = list[2];

    if domain != "github.com" {
        panic!("Invalid repository domain!");
    }

    if author.is_empty() || repository.is_empty() {
        panic!("{}", INVALID_URL);
    }

    if branch.is_empty() {
        panic!("No main branch specified!");
    }

    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        author, repository, branch
    );

    RepositoryInfo {
        url,
        author,
        name: repository,
    }
}

/// Check if specified directory exists.
/// Panics if it does not.
fn validate_dir(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        panic!("Path \"{}\" does not exist!", dir);
    }
}

/// Converts JSON data from the config file into
/// `ConfigFile` struct. Panics if fails to
/// either open the config file or to read it.
fn deserialise(path: &str) -> ConfigFile {
    let mut buf = Vec::new();
    let mut file = File::open(path).expect("Failed to open the config file");
    file.read_to_end(&mut buf)
        .expect("Failed to read the config file");

    serde_json::from_slice(&buf).expect("Failed to parse json config")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test URL formatter
    #[test]
    fn test_valid_url_fmt() {
        let url = "github.com/Makefolder/deployer";
        let branch = "master";
        let repository_info = url_fmt(url, branch);
        assert_eq!(
            repository_info.url,
            "https://api.github.com/repos/Makefolder/deployer/commits/master"
        );
        assert_eq!(repository_info.author, "Makefolder");
        assert_eq!(repository_info.name, "deployer");
    }

    #[test]
    #[should_panic(expected = "Invalid repository domain!")]
    fn test_invalid_domain_url_fmt() {
        let url = "gitlab.com/Makefolder/deployer";
        let branch = "master";
        url_fmt(url, branch);
    }

    #[test]
    #[should_panic(expected = "Invalid repository URL!")]
    fn test_invalid_author_url_fmt() {
        let url = "github.com//deployer";
        let branch = "master";
        url_fmt(url, branch);
    }

    #[test]
    #[should_panic(expected = "Invalid repository URL!")]
    fn test_invalid_name_url_fmt() {
        let url = "github.com/Makefolder/";
        let branch = "master";
        url_fmt(url, branch);
    }
}
