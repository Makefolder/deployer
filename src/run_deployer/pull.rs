use crate::generate_conf::file_struct::{Commit, ConfigFile};
use crate::log;
use build::build;
use chrono::{prelude::DateTime, Local};
use git2::Repository;
use reqwest::{Client, Response};
use std::{error::Error, fmt::Display, path::Path};
use tokio::time::{self, Duration};

mod build;
mod svc;

/// Local struct. Used to pass
/// these three fields across functions.
pub struct RepositoryInfo<'a> {
    pub url: String,
    pub author: &'a str,
    pub name: &'a str,
}

#[derive(Debug)]
pub enum FolderFormatError {
    FailedToFormat,
}

impl Error for FolderFormatError {}

impl Display for FolderFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            // I don't think this can happen but who knows, right?
            Self::FailedToFormat =>
                "Failed to format folder name. Try removing all repeating folders or try again in a minute.",
        };
        write!(f, "FolderFormatError: {message}")
    }
}

/// This function makes request to the GitHub's REST API.
/// Also builds "services" that are specified in the config file.
pub async fn ping<'a>(
    config: &ConfigFile,
    repository: &RepositoryInfo<'a>,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut last_commit = String::from("");
    loop {
        // Make request
        let res = send_request(&repository.url, &config.token, &client).await?;

        // Panic if an error occurred
        if !res.status().is_success() {
            let msg: String = format!("Failed to fetch data: {}", res.status());
            if res.status() == 401 {
                panic!("{}", msg);
            }
            continue;
        }

        let body = res.text().await?;
        let response: Commit = serde_json::from_str(&body)?;

        // Check for new commits
        if last_commit != response.sha {
            last_commit = response.sha.clone();
            let url = format!(
                "https://github.com/{}/{}.git",
                repository.author, repository.name
            );
            let pull_dir = format!("{}/{}", config.pull_dir, get_time());
            let pull_path = pull_repository(&url, &pull_dir)?;
            let path = Path::new(&pull_path);

            // todo: change hardcoded.
            //  Put into for s in services
            let build_dir = Path::new(&config.services[0].build_dir);
            build(path, &build_dir, config.services[0].name.to_owned())?;

            let svc_path = Path::new(&config.sys_svc_dir);
            let status = svc::restart_service(
                &config.services[0].svc_filename,
                &svc_path,
                &config.services[0].svc_file_contents,
            )?;
            if !status.success() {
                log!(
                    "Failed to restart service {} (status code {}).",
                    config.services[0].svc_filename,
                    status.code().unwrap_or(1)
                );
            }
        }
        time::sleep(Duration::from_secs(60)).await;
    }
}

async fn send_request(url: &str, token: &str, client: &Client) -> Result<Response, reqwest::Error> {
    let fmt_token = format!("token {}", token);
    let response = client
        .get(url)
        .header("Authorization", fmt_token)
        .header("User-Agent", "request")
        .send()
        .await?;
    Ok(response)
}

fn pull_repository(url: &str, root_dir: &str) -> Result<String, Box<dyn Error>> {
    // Pull repository
    match Repository::clone(url, root_dir) {
        Ok(_) => {
            log!("Fetched from remote branch to {}", root_dir);
            Ok(root_dir.to_string())
        }
        Err(e) => match e.code() {
            git2::ErrorCode::Exists => {
                let new_dest = update_destination(true, root_dir.to_owned(), 1)?;
                log!("updated destination: {}", new_dest);
                Repository::clone(url, &new_dest)?;
                Ok(new_dest)
            }
            _ => Err(Box::new(e)),
        },
    }
}

/// Recursive function. Checks if directory already exists
/// and appends available index to that folder so it is possible
/// to pull repository without issues.
///
/// Panics if fails to split
/// base path on underscores (_) into less than four pieces.
pub fn update_destination(
    exists: bool,
    mut path: String,
    index: i32,
) -> Result<String, FolderFormatError> {
    if !exists {
        return Ok(path);
    }
    let from_split = path.clone();
    let base_path: Vec<&str> = from_split.split("_").collect();
    destination_fmt(&base_path, &mut path, index)?;
    update_destination(check_existence(&path), path, index + 1)
}

/// External parameter `exists: bool` in `update_destination`
/// needed in function's tests. Therefore, I had to make this
/// function for `update_destination`'s internal use.
fn check_existence(path: &str) -> bool {
    let dir = Path::new(&path);
    dir.exists()
}

/// Append postfix to the path (example: `01_Jan_2025_1046_01`).
/// So `_01`, `_08`, `_12` is the result.
///
/// Panics if fails to split
/// base path on underscores (_) into less than four pieces.
fn destination_fmt(
    base_path: &[&str],
    path: &mut String,
    index: i32,
) -> Result<(), FolderFormatError> {
    if base_path.len() < 4 {
        return Err(FolderFormatError::FailedToFormat);
    }
    // assemble base of the destination path
    // "01" "Jan" "2025" "1046"
    *path = format!(
        "{}_{}_{}_{}",
        base_path[0], base_path[1], base_path[2], base_path[3]
    );
    path.push_str("_");
    if index < 10 {
        path.push('0');
    }
    path.push_str(&index.to_string());
    Ok(())
}

fn get_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%d_%b_%Y_%H%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_existent_path() {
        let non_existent_path = String::from("01_Sep_2024_1308");
        let result = update_destination(false, non_existent_path.clone(), 1);
        assert_eq!(result.unwrap(), non_existent_path);
    }

    #[test]
    fn test_invalid_path_format() {
        let invalid_path = String::from("01_Sep_2024");
        let result = update_destination(true, invalid_path, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_path_format() {
        let valid_path = String::from("01_Sep_2024_1307");
        let result = update_destination(true, valid_path, 1);

        // Ensure the format is correctly updated
        assert_eq!(result.unwrap(), "01_Sep_2024_1307_01");
    }
}
