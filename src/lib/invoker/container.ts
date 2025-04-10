import { invoke } from "@tauri-apps/api/core";
import {INVOKERS} from "$lib/invoker/utils";
import {ContainerInspectResponse, ContainerListResponseItem} from "the-moby-effect/MobySchemas";

export async function getContainers(onlyRunning : boolean = false): Promise<ContainerListResponseItem[]> {
    return invoke(INVOKERS.GET_CONTAINERS, {onlyRunning: onlyRunning})
}

export async function getContainerById(containerId: string): Promise<ContainerListResponseItem[]> {
    return invoke(INVOKERS.GET_CONTAINER, {containerId})
}

export async function inspectContainer(containerId: string): Promise<ContainerInspectResponse> {
    return invoke(INVOKERS.INSPECT_CONTAINER, {containerId})
}