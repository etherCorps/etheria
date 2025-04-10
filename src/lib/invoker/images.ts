import {ImageSummary} from "the-moby-effect/MobySchemas";
import {invoke} from "@tauri-apps/api/core";
import {INVOKERS} from "$lib/invoker/utils";

export async function getImages(onlyRunning : boolean = false): Promise<ImageSummary[]> {
    return invoke(INVOKERS.GET_IMAGES, {onlyRunning: onlyRunning})
}

export async function getImageInfo(name : string): Promise<ImageSummary> {
    return invoke(INVOKERS.GET_IMAGE_INFO, {name})
}