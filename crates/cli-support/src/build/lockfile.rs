//! Reading Cargo.lock lock file.

#![allow(clippy::new_ret_no_self)]

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};
use cargo_metadata::Metadata;
use console::style;
use semver::{Version, VersionReq};
use serde::Deserialize;

/// This struct represents the contents of `Cargo.lock`.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Lockfile {
    package: Vec<Package>,
    root_package_name: Option<String>,
}

/// This struct represents a single package entry in `Cargo.lock`
#[derive(Clone, Debug, Deserialize)]
struct Package {
    name: String,
    version: Version,
    dependencies: Option<Vec<String>>,
}

pub enum DepCheckError {
    VersionError(String, Option<Version>),
    Error(anyhow::Error),
}

impl Lockfile {
    /// Read the `Cargo.lock` file for the crate at the given path.
    pub fn new(crate_data: &Metadata) -> Result<Lockfile> {
        let lock_path = get_lockfile_path(crate_data)?;
        let lockfile = fs::read_to_string(&lock_path)
            .with_context(|| anyhow!("failed to read: {}", lock_path.display()))?;
        let mut lockfile: Lockfile = toml::from_str(&lockfile)
            .with_context(|| anyhow!("failed to parse: {}", lock_path.display()))?;
        lockfile.root_package_name = crate_data.root_package().map(|p| p.name.to_string());
        Ok(lockfile)
    }

    /// Obtains and verifies the given library matches the given semver version
    /// Min version is used for the semver comparison check
    /// Cur version is only used for help text
    /// Errors with the wrong version if incorrect.
    pub fn require_lib(
        &self,
        lib_name: &str,
        min_version: &Version,
        cur_version: &Version,
    ) -> Result<(), DepCheckError> {
        let req = VersionReq::parse(&format!("^{min_version}")).unwrap();
        if let Some(version) = self
            .get_package_version(lib_name)
            .map_err(DepCheckError::Error)?
        {
            if !req.matches(&version) {
                return Err(DepCheckError::VersionError(
                    format!(
                        "Unsupported version {}, expected at least {}",
                        style(format!("{lib_name}@{version}")).bold().red(),
                        cargo_dep_error(lib_name, cur_version)
                    ),
                    Some(version),
                ));
            }
        } else {
            return Err(DepCheckError::VersionError(
                format!(
                    "Ensure that you have dependency {}",
                    cargo_dep_error(lib_name, cur_version)
                ),
                None,
            ));
        }
        Ok(())
    }

    /// Obtains the package version for the given package
    /// If there are multiple matching packages, and there is a root package,
    /// returns the package matching the root package name only, otherwise returns the first one.
    fn get_package_version(&self, package: &str) -> Result<Option<Version>> {
        // If we have a root package, use the exact version if it has an exact version inlined into deps
        if let Some(root_package_name) = &self.root_package_name {
            if let Some(root_pkg) = self.package.iter().find(|p| p.name == *root_package_name) {
                if let Some(dependencies) = &root_pkg.dependencies {
                    for dep in dependencies.iter() {
                        if dep.starts_with(package)
                            && dep.chars().nth(package.len() + 1) == Some(' ')
                        {
                            let version = &dep[package.len() + 1..];
                            if !version.is_empty() {
                                return Ok(Some(Version::parse(version)?));
                            }
                        }
                    }
                }
            }
        }
        // Otherwise take the first matching package name to get the version
        Ok(self
            .package
            .iter()
            .find(|p| p.name == package)
            .map(|p| p.version.clone()))
    }
}

fn cargo_dep_error(lib_name: &str, cur_version: &Version) -> String {
    format!(
        "{} in the Cargo.toml file:\n\n\
         [dependencies]\n\
         {lib_name} = \"{}\"",
        style(format!("{lib_name}@{}", cur_version)).bold().green(),
        *cur_version,
    )
}

/// Given the path to the crate that we are building, return a `PathBuf`
/// containing the location of the lock file, by finding the workspace root.
fn get_lockfile_path(crate_data: &Metadata) -> Result<PathBuf> {
    // Check that a lock file can be found in the directory. Return an error
    // if it cannot, otherwise return the path buffer.
    let lockfile_path = crate_data.workspace_root.join("Cargo.lock");
    if !lockfile_path.is_file() {
        bail!("Could not find lockfile at {:?}", lockfile_path)
    } else {
        Ok(lockfile_path.into())
    }
}
