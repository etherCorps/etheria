use crate::docker::start::connect_to_docker;
use bollard::Docker;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Clone)]
pub struct InstallationStatus {
    pub docker_installed: bool,
    pub colima_installed: bool,
}

#[derive(Debug)]
pub struct AppState {
    pub docker: Option<Docker>,
    pub is_docker_connected: bool,
    pub backend_initialized: bool,
    pub cancel_stats: Arc<AtomicBool>,
    pub cancel_logs: Arc<AtomicBool>,
}

pub type SharedAppState = Arc<Mutex<AppState>>;

impl Display for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            docker: None,
            is_docker_connected: false,
            backend_initialized: false,
            cancel_stats: Arc::new(AtomicBool::new(false)),
            cancel_logs: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn connect_docker(&mut self) -> Result<(), String> {
        log::info!("Connecting to Docker...");
        if self.is_docker_connected {
            return Ok(());
        }
        match connect_to_docker().await {
            Ok(docker) => {
                self.docker = Some(docker);
                self.is_docker_connected = true;
                log::info!("Connected to docker");
                Ok(())
            }
            Err(err) => {
                eprintln!("Error connecting to Docker: {}", err); // Log the error
                Err(err.to_string())
            }
        }
    }

    pub fn get_docker(&self) -> Result<&Docker, String> {
        self.docker
            .as_ref()
            .ok_or_else(|| "Docker is not connected".to_string())
    }
}
