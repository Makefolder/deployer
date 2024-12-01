// SVC stands for service files (.service)
// in Ubuntu those are located in
// /lib/systemd/system/

use std::{
    fs,
    io::{Result, Write},
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
    let dir = complete_dir(svc_filename, svc_dir);
    dbg!("Service file in {}", dir.display());
    match dir.exists() {
        false => {
            let mut file = fs::OpenOptions::new().create(true).append(true).open(dir)?;
            for s in contents {
                _ = file.write(&s.as_bytes());
                _ = file.write(b"\n");
            }
            let mut cmd = Command::new("systemctl")
                .arg("restart")
                .arg(svc_filename)
                .spawn()
                .expect("Failed to restart service");
            cmd.wait()
        }
        true => {
            let mut cmd = Command::new("systemctl")
                .arg("restart")
                .arg(svc_filename)
                .spawn()
                .expect("Failed to restart service");
            cmd.wait()
        }
    }
}

fn complete_dir(svc_filename: &str, svc_dir: &Path) -> PathBuf {
    let dir_str = svc_dir.to_str().unwrap();
    let assembled = format!("{}/{}", dir_str, svc_filename);
    PathBuf::from(assembled)
}
