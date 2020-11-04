use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use regex::Regex;
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::model::package::{Package, PackageIndex};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};

pub(crate) trait CacheTrait {
    fn update(&self) -> Result<PathBuf>;
    fn get_package(&self, name: &str) -> Result<Package>;
    fn list_packages(&self, pattern: &str, owner: &str) -> Result<Vec<Package>>;
    fn has_package(&self, name: &str) -> Result<bool>;
    fn get_package_indexes(&self) -> Result<Vec<PackageIndex>>;
}

#[derive(Debug)]
pub(crate) struct CacheInfo {
    location: String,
}

#[derive(Debug)]
pub(crate) struct CacheService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl CacheService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }
}

impl CacheTrait for CacheService {
    fn update(&self) -> Result<PathBuf> {
        let config = self.config.as_ref().unwrap();
        let dir = config.huber_repo_dir()?;
        let runtime = self.runtime.as_ref().unwrap();

        runtime.block_on(async {
            let client = GithubClient::new(
                config.github_credentials.clone(),
                config.git_ssh_key.clone(),
            );
            client.clone("innobead", "huber", dir.clone()).await
        })?;

        Ok(dir)
    }

    fn get_package(&self, name: &str) -> Result<Package> {
        if !self.has_package(name)? {
            return Err(anyhow!("{} not found", name));
        }

        let config = self.config.as_ref().unwrap();
        let pkg_file = config.managed_pkg_manifest_file(name)?;
        let pkg = serde_yaml::from_reader::<File, Package>(File::open(pkg_file)?)?;

        Ok(pkg)
    }

    fn list_packages(&self, pattern: &str, owner: &str) -> Result<Vec<Package>> {
        let mut pkgs: Vec<Package> = vec![];

        match pattern {
            "" => {
                for p in self.get_package_indexes()? {
                    if owner == "" {
                        pkgs.push(self.get_package(&p.name)?);
                        continue;
                    }

                    if p.owner == owner {
                        pkgs.push(self.get_package(&p.name)?);
                    }
                }
            }

            _ => {
                let regex = Regex::new(pattern)?;

                for p in self.get_package_indexes()? {
                    if regex.is_match(&p.name) {
                        pkgs.push(self.get_package(&p.name)?);
                    }
                }
            }
        }

        Ok(pkgs)
    }

    fn has_package(&self, name: &str) -> Result<bool> {
        Ok(self.get_package_indexes()?.iter().any(|it| it.name == name))
    }

    fn get_package_indexes(&self) -> Result<Vec<PackageIndex>> {
        let config = self.config.as_ref().unwrap();
        let index_file = config.managed_pkg_index_file()?;
        let pkg_indexes =
            serde_yaml::from_reader::<File, Vec<PackageIndex>>(File::open(index_file)?)?;

        Ok(pkg_indexes)
    }
}
