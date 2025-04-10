use crate::state::SharedAppState;
use bollard::image::{ListImagesOptions, RemoveImageOptions};
use bollard::models::{HistoryResponseItem, ImageDeleteResponseItem, ImageInspect, ImageSummary};

#[tauri::command]
pub async fn get_images(
    state: tauri::State<'_, SharedAppState>,
) -> Result<Vec<ImageSummary>, String> {
    let options = Some(ListImagesOptions {
        all: true,
        ..Default::default()
    });

    // Isolate Mutex access
    let docker = {
        let state = state.lock().await;
        state.get_docker()?.clone() // Clone the docker instance.
    };

    docker
        .list_images::<String>(options)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_image_info(
    state: tauri::State<'_, SharedAppState>,
    name: String,
) -> Result<ImageInspect, String> {
    // Isolate Mutex access
    let docker = {
        let state = state.lock().await;
        state.get_docker()?.clone() // Clone the docker instance.
    };

    docker.inspect_image(&name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_image_history(
    state: tauri::State<'_, SharedAppState>,
    name: String,
) -> Result<Vec<HistoryResponseItem>, String> {
    // Isolate Mutex access
    let docker = {
        let state = state.lock().await;
        state.get_docker()?.clone() // Clone the docker instance.
    };

    docker.image_history(&name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_image(
    state: tauri::State<'_, SharedAppState>,
    image_name: String,
    force: bool,
    no_prune: bool,
) -> Result<Vec<ImageDeleteResponseItem>, String> {
    let remove_options = Some(RemoveImageOptions {
        force,
        noprune: no_prune,
        ..Default::default()
    });

    // Isolate Mutex access
    let docker = {
        let state = state.lock().await;
        state.get_docker()?.clone() // Clone the docker instance.
    };

    docker
        .remove_image(&image_name, remove_options, None)
        .await
        .map_err(|e| e.to_string())
}
