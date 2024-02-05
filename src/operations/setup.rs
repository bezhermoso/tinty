use crate::config::Config;
use crate::constants::REPO_DIR;
use crate::utils::git_clone;
use anyhow::Result;
use std::path::Path;

/// Sets up the base16-shell-manager repository at the specified path.
///
/// This function checks if the repository path already exists. If it does, it prints a message indicating
/// that the repository is already set up and suggests using the `update` subcommand for updates. If the
/// repository path does not exist, it proceeds to clone and set up the base16-shell-manager repository at the given path.
pub fn setup(config_path: &Path, data_path: &Path) -> Result<()> {
    let config = Config::read(config_path)?;
    let items = config.items.unwrap_or_default();
    let hooks_path = data_path.join(REPO_DIR);

    for item in items {
        let item_path = hooks_path.join(&item.name);

        if !item_path.is_dir() {
            git_clone(item.git_url.as_str(), &item_path)?;

            println!("{} installed", item.name);
        } else {
            println!("{} already installed", item.name);
        }
    }

    Ok(())
}
