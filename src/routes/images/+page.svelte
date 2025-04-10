<script lang="ts">
    import type {PageProps} from "./$types";
    import {type ImageSummary} from "the-moby-effect/MobySchemas";
    import {onMount} from "svelte";
    import {pageState} from "$lib/stores.svelte";
    import {getImageInfo} from "$lib/invoker/images";

    let {data}: PageProps = $props();
    // let images: Array<ImageSummary> = $state(data.images) as Array<ImageSummary>

    onMount(async () => {
        for (const image of data.images) {
            console.log(image);
            console.log(await getImageInfo(image!.RepoTags?.[0]));
        }
    })
</script>