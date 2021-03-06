pub mod namespaces;
#[cfg(all(unix, not(target_os = "redox")))]
mod library_iter;
#[cfg(all(unix, not(target_os = "redox")))]
pub use self::library_iter::*;

use app_dirs::{AppDataType, AppInfo, app_root};
use std::path::PathBuf;

pub fn config_dir() -> Option<PathBuf> {
    match app_root(
        AppDataType::UserConfig,
        &AppInfo {
            name: "ion",
            author: "Redox OS Developers",
        },
    ) {
        Ok(mut path) => {
            path.push("plugins");
            Some(path)
        }
        Err(why) => {
            eprintln!("ion: unable to get config directory: {:?}", why);
            None
        }
    }
}
