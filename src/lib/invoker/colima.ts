import {invoke} from "@tauri-apps/api/core";
import {INVOKERS} from "$lib/invoker/utils";

export async function startColimaService() {
    await invoke(INVOKERS.START_COLIMA_SERVICE)
}

export async function stopColimaService () {
    await invoke(INVOKERS.STOP_COLIMA_SERVICE)
}

export async function connectToDocker() {
    await invoke(INVOKERS.CONNECT_TO_DOCKER)
}