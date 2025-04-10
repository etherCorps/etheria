use crate::state::SharedAppState;
use serde::{Deserialize, Serialize};
use std::process::{Command, Output, Stdio};
use tauri::State;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ColimaStatus {
    #[serde(rename = "isDockerInstalled")]
    pub is_docker_installed: bool,
    #[serde(rename = "isColimaInstalled")]
    pub is_colima_installed: bool,
    #[serde(rename = "isColimaRunning")]
    pub is_colima_running: bool,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DockerStatus {
    #[serde(rename = "isDockerRunning")]
    pub is_docker_running: bool,
}

#[tauri::command]
pub async fn check_prerequisites(state: State<'_, SharedAppState>) -> Result<ColimaStatus, String> {
    let is_docker_installed = Command::new("docker").arg("--version").output().is_ok();

    let is_colima_installed = Command::new("colima").arg("--version").output().is_ok();
    let colima_status_run = check_colima_status();
    let is_colima_running = match colima_status_run.ok() {
        Some(None) => false,
        Some(_String) => true,
        _ => false,
    };

    let new_status = ColimaStatus {
        is_docker_installed,
        is_colima_installed,
        is_colima_running,
    };

    Ok(new_status)
}

pub fn check_colima_status() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let output: Output = Command::new("colima").arg("status").output()?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        print!("stdout {:#?}", stdout);
        // Now we check the stdout for the running message
        if stdout.contains("colima is running") {
            Ok(Some(stdout))
        } else {
            // colima command was successful but colima is not running
            Ok(Some(stdout))
        }
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        if stderr.contains("colima is not running") || stderr.contains("colima is not installed") {
            Ok(None)
        } else {
            Err(format!("Colima status command failed: {}", stderr).into())
        }
    }
}

#[tauri::command]
pub async fn start_colima_service(
    state: State<'_, SharedAppState>,
) -> Result<ColimaStatus, String> {
    log::info!("Starting Colima...");
    // Start Colima
    Command::new("colima")
        .arg("start")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to start colima: {}", e))
        .expect("TODO: panic message");

    // Isolate Mutex access
    let docker = {
        let mut state = state.lock().await;
        state.connect_docker().await
    };

    // Check Colima status
    Command::new("colima")
        .arg("status")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to get colima status: {}", e))
        .expect("TODO: panic message");

    let new_status = ColimaStatus {
        is_docker_installed: true,
        is_colima_installed: true,
        is_colima_running: true,
    };

    Ok(new_status)
}

#[tauri::command]
pub async fn stop_colima_service(state: State<'_, SharedAppState>) -> Result<ColimaStatus, String> {
    log::info!("Stopping Colima...");
    // Start Colima
    Command::new("colima")
        .arg("stop")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to start colima: {}", e))
        .expect("TODO: panic message");

    state.lock().await.is_docker_connected = false;

    let new_status = ColimaStatus {
        is_docker_installed: true,
        is_colima_installed: true,
        is_colima_running: false,
    };

    Ok(new_status)
}

#[tauri::command]
pub async fn connect_to_docker(state: State<'_, SharedAppState>) -> Result<DockerStatus, String> {
    let mut state = state.lock().await;
    state.connect_docker().await.expect("TODO: panic message");

    let status = DockerStatus {
        is_docker_running: state.is_docker_connected,
    };
    Ok(status)
}
