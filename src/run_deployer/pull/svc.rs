// SVC stands for service files (.service)
// in Ubuntu those are located in
// /lib/systemd/system/

use crate::log;
use chrono::{prelude::DateTime, Local};
use std::{
    fs,
    io::{ErrorKind, Result, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

/// Tries to restart `svc_filename` service.
/// If service is not found/failed to restart
/// it would go to `sys_svc_dir` and create a
/// new one
pub fn restart_service(
    svc_filename: &str,
    svc_dir: &Path,
    contents: &[String],
) -> Result<ExitStatus> {
    let dir = complete_dir(svc_dir, svc_filename);
    match dir.exists() {
        false => {
            let mut file = fs::OpenOptions::new().create(true).append(true).open(dir)?;
            for s in contents {
                _ = file.write(&s.as_bytes());
                _ = file.write(b"\n");
            }
            let cmd = Command::new("systemctl")
                .arg("start")
                .arg(svc_filename)
                .spawn();
            if let Ok(mut c) = cmd {
                c.wait()
            } else {
                log!("Failed to use `systemctl` command.");
                Err(ErrorKind::Unsupported.into())
            }
        }
        true => {
            let cmd = Command::new("systemctl")
                .arg("restart")
                .arg(svc_filename)
                .spawn();
            if let Ok(mut c) = cmd {
                c.wait()
            } else {
                log!("Failed to use `systemctl` command.");
                Err(ErrorKind::Unsupported.into())
            }
        }
    }
}

fn complete_dir(svc_dir: &Path, svc_filename: &str) -> PathBuf {
    let dir_str = svc_dir.to_str().unwrap();
    let assembled = format!("{}/{}", dir_str, svc_filename);
    PathBuf::from(assembled)
}
