pub mod directory;
pub mod xfce;
use std::collections::BTreeMap;
#[derive(Default, Debug)]
pub struct SystemInfo {
    pub hostname: String,
    pub distribution: String,
    pub kernel: String,
    pub cpu: String,
    pub memory: String,
    pub session: String,
    pub desktop: String,
    pub xfce: BTreeMap<String, String>,
    pub integration: String,
}
pub trait SettingsBackend: Send + Sync {
    fn read_system(&self) -> SystemInfo;
}
// All subprocess operations are bounded, avoid shells, and belong to backends only.
pub fn command(program: &str, args: &[&str]) -> Result<String, String> {
    let out = std::process::Command::new("timeout")
        .args(["8s", program])
        .args(args)
        .env("LC_ALL", "C")
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(format!("{program}: {}", out.status))
    }
}
