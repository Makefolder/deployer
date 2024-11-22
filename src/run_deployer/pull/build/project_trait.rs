use std::io::Result;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub trait Project {
    /// Panics if fails to spawn the CMD.
    fn build(&self, current_dir: &Path) -> Result<ExitStatus>;
    fn get_build_dir(&self) -> &str;
}

// RUST
pub struct Rust<'a> {
    build_dir: &'a str,
}

impl<'a> Rust<'a> {
    pub fn new() -> Self {
        Rust {
            build_dir: "target/release",
        }
    }
}

impl<'a> Project for Rust<'a> {
    fn build(&self, current_dir: &Path) -> Result<ExitStatus> {
        let mut cmd = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(current_dir)
            .spawn()
            .expect("Failed to build Rust project");
        cmd.wait()
    }

    fn get_build_dir(&self) -> &str {
        self.build_dir
    }
}

// GO
pub struct Go<'a> {
    build_dir: &'a str,
}

impl<'a> Go<'a> {
    pub fn new() -> Self {
        // it builds in root dir of a project
        Go { build_dir: "" }
    }
}

impl<'a> Project for Go<'a> {
    fn build(&self, current_dir: &Path) -> Result<ExitStatus> {
        let mut cmd = Command::new("go")
            .arg("build")
            .arg(".")
            .current_dir(current_dir)
            .spawn()
            .expect("Failed to build Go project");
        cmd.wait()
    }

    fn get_build_dir(&self) -> &str {
        self.build_dir
    }
}

// GLEAM
pub struct Gleam<'a> {
    build_dir: &'a str,
}

impl<'a> Gleam<'a> {
    pub fn new() -> Self {
        Gleam {
            build_dir: "build/prod/erlang",
        }
    }
}

impl<'a> Project for Gleam<'a> {
    fn build(&self, _current_dir: &Path) -> Result<ExitStatus> {
        todo!()
    }

    fn get_build_dir(&self) -> &str {
        self.build_dir
    }
}
