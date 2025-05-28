mod github;

use semver::Version;
use anyhow::Result;

#[derive(Clone)]
pub struct UpdateChecker {
    current_version: Version,
}

#[derive(Debug)]
pub struct UpdateStatus {
    pub current_version: Version,
    pub latest_version: Option<Version>,
    pub update_available: bool,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            current_version: Version::new(0, 1, 0)  // Match Cargo.toml version
        }
    }

    pub async fn check_for_updates(&self) -> Result<UpdateStatus> {
        let latest_version = github::fetch_latest_version().await?;
        
        Ok(UpdateStatus {
            current_version: self.current_version.clone(),
            latest_version: Some(latest_version.clone()),
            update_available: latest_version > self.current_version,
        })
    }
}

