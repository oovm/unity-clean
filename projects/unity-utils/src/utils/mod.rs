use std::path::Path;

pub fn is_unity_path(dir: &Path) -> bool {
    let assets = dir.join("Assets");
    if !assets.exists() || !assets.is_dir() {
        return false;
    }
    let packages = dir.join("Packages");
    if !packages.exists() || !packages.is_dir() {
        return false;
    }
    let project_settings = dir.join("ProjectSettings");
    if !project_settings.exists() || !project_settings.is_dir() {
        return false;
    }
    return true;
}
