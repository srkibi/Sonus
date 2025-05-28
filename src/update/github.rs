use anyhow::{Result, Context};
use semver::Version;
use serde::Deserialize;

const GITHUB_API_URL: &str = "https://api.github.com/repos/kibi/sonus/releases/latest";

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

pub(crate) async fn fetch_latest_version() -> Result<Version> {
    let client = reqwest::Client::new();
    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "Sonus-DAW")
        .send()
        .await
        .context("Failed to fetch GitHub release info")?;

    let release: GitHubRelease = response
        .json()
        .await
        .context("Failed to parse GitHub response")?;

    let version_str = release.tag_name.trim_start_matches('v');
    Version::parse(version_str)
        .context("Failed to parse version from GitHub release tag")
}

