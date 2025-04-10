import {invoke} from "@tauri-apps/api/core";
import {INVOKERS} from "$lib/invoker/utils";

export type ColimaStatus = {
    isDockerInstalled: boolean
    isColimaInstalled: boolean
    isColimaRunning: boolean
}

export async function getInstallationState(): Promise<ColimaStatus> {
    return invoke(INVOKERS.CHECK_PREREQUISITES)
}
