use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use directories::ProjectDirs;

const PROJECT_CONFIG_PARAMS: (&str, &str, &str) = ("com", "YourCompany", "GameEditor");

pub fn save_json_config(file: &str, contents: String) {
    let Ok(_) = ensure_config_dir() else {
        println!("Could not create config directory.");
        return;
    };

    let result = write_json_config(file, contents);
    match result {
        Ok(_) => println!("Saved {} config file", file),
        Err(e) => println!("Error saving {} file: {}", file, e.kind()),
    }
}

pub fn read_json_config(file: &str) -> std::io::Result<String> {
    let mut file = File::open(json_config(file))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn proj_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from(
        PROJECT_CONFIG_PARAMS.0,
        PROJECT_CONFIG_PARAMS.1,
        PROJECT_CONFIG_PARAMS.2,
    )
}

fn ensure_config_dir() -> std::io::Result<()> {
    let proj_dirs = proj_dirs().unwrap();
    fs::create_dir_all(proj_dirs.config_local_dir())?;
    Ok(())
}

fn json_config(file: &str) -> PathBuf {
    proj_dirs()
        .unwrap()
        .config_local_dir()
        .join(file.to_owned() + ".json")
}

fn write_json_config(file: &str, contents: String) -> std::io::Result<()> {
    let mut file = File::create(json_config(file))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
