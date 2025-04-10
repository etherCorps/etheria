<script lang="ts">
    import * as Card from "$lib/components/shadcn/card"
    import { writeText } from '@tauri-apps/plugin-clipboard-manager';
    import type {ContainerListResponseItem} from "the-moby-effect/MobySchemas";
    import {Button} from "$lib/components/shadcn/button";
    import {Copy, Play} from "@lucide/svelte";
    import { toast } from "svelte-sonner";
    import {kebabToNormal} from "$lib/helpers/convertors";

    let {container}: { container: ContainerListResponseItem } = $props()

    async function copyContainerId(id: string) {
        await writeText(id)
        toast.success(`Successfully copied container ID of ${kebabToNormal(container?.Names?.[0] ?? '')}`)
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title class="flex flex-row items-center justify-between w-full gap-x-2">
            <span class="truncate">{kebabToNormal(container?.Names?.[0] ?? "container")}</span>
            <div class="flex flex-row items-center gap-x-2">
                <Button variant="ghost" size="sm" class="size-8">
                    <Play />
                </Button>
            </div>
        </Card.Title>
        <Card.Description class="flex flex-row items-center w-full">
            <span class="truncate w-1/4">
                {container.Id}
            </span>
            <Button variant="ghost" size="icon" class="[&_svg]:size-3 size-6" onclick={() => copyContainerId(container.Id)}>
                <Copy />
            </Button>
        </Card.Description>
    </Card.Header>
    <Card.Content>
        <p>Card Content</p>
    </Card.Content>
    <Card.Footer>
        <p>Card Footer</p>
    </Card.Footer>
</Card.Root>