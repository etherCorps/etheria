use bollard::errors::Error;
use bollard::{Docker, API_DEFAULT_VERSION};
use std::env;
use std::path::PathBuf;

pub async fn connect_to_docker() -> Result<Docker, Error> {
    let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
    log::info!("WTF: {}", home_dir);
    let colima_socket = PathBuf::from(home_dir)
        .join(".colima")
        .join("default")
        .join("docker.sock");
    Docker::connect_with_unix(&colima_socket.to_string_lossy(), 120, API_DEFAULT_VERSION)
}
