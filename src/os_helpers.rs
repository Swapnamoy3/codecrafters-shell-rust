use std::env;
use std::path::PathBuf;
use std::fs;



#[cfg(unix)] // This ensures the following code only compiles on Unix-like systems
use std::os::unix::fs::PermissionsExt;

pub fn is_executable(file: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        file.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    {
        file.is_file()
    }
}

pub fn get_path() -> Vec<PathBuf>{
    let paths = env::split_paths(&env::var_os("PATH").unwrap_or_default()).collect::<Vec<PathBuf>>();

    paths
}