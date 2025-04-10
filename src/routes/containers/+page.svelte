<script lang="ts">
    import {ContainerListResponseItem} from "the-moby-effect/MobySchemas";
    import ContainerCard from "$lib/components/ui/containers/container-card.svelte";
    import type {PageProps} from "./$types";
    import {pageState} from "$lib/stores.svelte";
    import {LoaderCircle, Container} from "@lucide/svelte";
    import {Button} from "$lib/components/shadcn/button";

    let {data}: PageProps = $props();
    let containers: Array<ContainerListResponseItem> = $state(data.containers) as Array<ContainerListResponseItem>
</script>

<div class="w-full h-full">
    {#if pageState.isLoading}
        <div class="flex flex-row items-center justify-center h-full gap-x-2">
            <LoaderCircle class="size-8 animate-spin" />
            <p class="text-lg">Loading containers</p>
        </div>
    {:else if containers.length === 0}
        <div class="flex flex-row items-center justify-center h-full w-full">
            <Button href="/images">
                <Container />
                Create a new container
            </Button>
        </div>
    {:else }
        <div class="grid grid-cols-2 gap-3">
            {#if containers && containers.length > 0}
                {#each containers as container (container.Id)}
                    <ContainerCard {container}/>
                {/each}
            {/if}
        </div>
    {/if}
</div>

