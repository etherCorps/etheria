use crate::state::SharedAppState;
use bollard::container::ListContainersOptions;
use bollard::models::{ContainerInspectResponse, ContainerSummary};
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn get_containers(
    state: State<'_, SharedAppState>,
    only_running: Option<bool>,
) -> Result<Vec<ContainerSummary>, String> {
    let all = !only_running.unwrap_or(true);

    let opts = ListContainersOptions::<String> {
        all,
        ..Default::default()
    };

    // Isolate Mutex access
    let docker = {
        let state = state.lock().await;
        state.get_docker()?.clone() // Clone the docker instance.
    };

    // Now release the lock, and then await.
    docker
        .list_containers(Some(opts))
        .await
        .map_err(|e| format!("Failed to list containers: {}", e))
}

#[tauri::command]
pub async fn get_container_by_id(
    state: State<'_, SharedAppState>,
    container_id: String,
) -> Result<ContainerSummary, String> {
    let mut list_container_filters = HashMap::new();
    list_container_filters.insert(String::from("id"), vec![container_id]);

    let opts = ListContainersOptions::<String> {
        all: true,
        filters: list_container_filters,
        ..Default::default()
    };

    let docker = {
        let state = state.lock().await;
        state.get_docker()?.clone()
    };

    docker
        .list_containers(Some(opts))
        .await
        .map_err(|e| format!("Failed to list containers: {}", e))?
        .into_iter()
        .next()
        .ok_or_else(|| "Container not found".to_string())
}

#[tauri::command]
pub async fn fetch_container_info(
    state: State<'_, SharedAppState>,
    container_id: String,
) -> Result<ContainerInspectResponse, String> {
    let docker = {
        let state = state.lock().await;
        state.get_docker()?.clone()
    };

    docker
        .inspect_container(container_id.as_str(), None)
        .await
        .map_err(|e| format!("Failed to inspect container: {}", e))
}
