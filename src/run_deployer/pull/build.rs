// Parse services, search for "key-files", build the project.
// The "key-files" are files that are important for the project
// such as "package.json", "gleam.toml" or "Cargo.toml".

use crate::log;
use crate::run_deployer::pull::{DateTime, Local};
use project_trait::{Go, Project, Rust};
use std::process::Command;
use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
    path::Path,
    process::ExitStatus,
};
use walkdir::{DirEntry, WalkDir};

mod project_trait;

enum KeyFile {
    //Gleam,
    //NodeJS,
    //Elixir,
    Rust,
    Go,
}

// The filenames of KeyFiles
// Used in list_directories() and KeyFile::value(&self)
//const NODEJS: &str = "package.json";
//const GLEAM: &str = "gleam.toml";
//const ELIXIR: &str = "TODO";
const CARGO: &str = "Cargo.toml";
const GO_MOD: &str = "go.mod";

impl KeyFile {
    fn value(&self) -> &str {
        match self {
            //KeyFile::Elixir => ELIXIR,
            //KeyFile::NodeJS => NODEJS,
            //KeyFile::Gleam => GLEAM,
            KeyFile::Rust => CARGO,
            KeyFile::Go => GO_MOD,
        }
    }
}

impl Display for KeyFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Build a service looking at its `KeyFiles`.
pub fn build(service_path: &Path, build_dir: &Path, service_name: &str) -> Result<()> {
    let key_file = list_directories(service_path)?;
    log!(
        "Found a key file ({}) in {}",
        key_file.1,
        key_file.0.path().display()
    );
    let path = key_file
        .0
        .path()
        .parent()
        .expect("Failed to get file's parent directory");

    match key_file.1 {
        KeyFile::Rust => {
            let rust = Rust::new();
            let status = rust.build(path);

            log!(
                "Build command has finished with status: {}",
                status.expect("Failed to get exit status code")
            );

            let p = path.to_str().expect("Failed to get rust build path");
            let rs_build_path = format!("{}/{}", p, rust.get_build_dir());
            move_build(Path::new(&rs_build_path), build_dir, service_name)?;
        }
        KeyFile::Go => {
            let /* me */ go /* it's 12:30AM already :( */ = Go::new();
            let status = go.build(path);
            let project_name = go
                .get_build_name(key_file.0.path())
                .expect("Failed to find Go module name");

            log!(
                "Build command has finished with {}",
                status.expect("Failed to get exit status code")
            );

            let go_build_path = if let Some(module) = project_name {
                format!(
                    "{}/{}{}",
                    path.to_str().expect("Failed to get Go build path"),
                    go.get_build_dir(),
                    module
                )
            } else {
                format!(
                    "{}/{}",
                    path.to_str().expect("Failed to get Go build path"),
                    go.get_build_dir()
                )
            };

            move_build(Path::new(&go_build_path), build_dir, service_name)?;
        } //KeyFile::Gleam => {
          //    let gleam = Gleam::new();
          //    let status = gleam.build(path);
          //    log!(
          //        "Build command has finished with status: {}",
          //        status.expect("Failed to get exit status code")
          //    );
          //    let go_build_path = format!(
          //        "{}/{}",
          //        gleam.get_build_dir(),
          //        path.to_str().expect("Failed to get go build path")
          //    );
          //    move_build(Path::new(&go_build_path), build_dir, service_name)?;
          //}
          //KeyFile::Elixir => todo!(),
          //KeyFile::NodeJS => {
          //    // NodeJS backend doesn't compile. You deploy as is.
          //    // **unless you deploy frontend**
          //    move_build(path, build_dir, service_name)?;
          //}
    }
    Ok(())
}

/// Moves and renames the build.
fn move_build(project: &Path, destination: &Path, service_name: &str) -> Result<ExitStatus> {
    let tmp = format!("{}/{}", destination.to_str().unwrap(), service_name);
    let destination = Path::new(&tmp);
    if Path::exists(destination) {
        let mut cmd = Command::new("rm")
            .arg("-rf")
            .arg(destination)
            .spawn()
            .expect("Failed to remove existing service.");
        cmd.wait()?;
    }
    let mut cmd = Command::new("mv")
        .arg(project)
        .arg(destination)
        .spawn()
        .expect("Failed to move release");
    cmd.wait()
}

/// Search for supported `KeyFiles`.
fn list_directories(path: &Path) -> Result<(DirEntry, KeyFile)> {
    for entry in WalkDir::new(path).follow_links(true).into_iter() {
        let tmp = entry?;
        if tmp.path().is_file() {
            let file_name = tmp.path().file_name().unwrap().to_str().unwrap();
            match file_name {
                //NODEJS => return Ok((tmp, KeyFile::NodeJS)),
                //GLEAM => return Ok((tmp, KeyFile::Gleam)),
                //ELIXIR => return Ok((tmp, KeyFile::Elixir)),
                CARGO => return Ok((tmp, KeyFile::Rust)),
                GO_MOD => return Ok((tmp, KeyFile::Go)),
                _ => continue,
            }
        }
    }
    Err(Error::new(
        ErrorKind::Other,
        "Couldn't find any supported key-file.",
    ))
}
