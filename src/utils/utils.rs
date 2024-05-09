pub fn check_if_directory_empty(dir_path: String) -> Result<bool, String> {
    let dir = std::fs::read_dir(dir_path);
    match dir {
        Ok(mut dir) => return Ok(dir.next().is_none()),
        Err(_) => {
            return Ok(true);
        }
    }
}

pub fn check_if_package_manager_installed(pkgm: &str) -> Result<bool, String> {
    let output = std::process::Command::new("which")
        .arg(pkgm)
        .output()
        .expect("Failed to execute 'which' command");

    let is_installed = output.status.success();

    Ok(is_installed)
}
