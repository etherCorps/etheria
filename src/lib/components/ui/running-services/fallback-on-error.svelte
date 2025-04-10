<script lang="ts">
    import { Button } from "$lib/components/shadcn/button/index.js";
    import * as Card from "$lib/components/shadcn/card/index.js";
    import { Switch } from "$lib/components/shadcn/switch/index.js";
    import {type ColimaStatus} from "$lib/invoker/installation";
    import {Container, Download, Icon, Power, ServerCog, ServerCrash, LoaderCircle, Check} from "@lucide/svelte"
    import {invalidateAll} from "$app/navigation";
    import {startColimaService} from "$lib/invoker/colima";
    import {toast} from "svelte-sonner";

    let status: ColimaStatus = $props()

    let loadingState: Record<keyof ColimaStatus, boolean> = $state({
        isColimaRunning: false,
        isColimaInstalled: false,
        isDockerInstalled: false
    })

    type CheckerItem = {
        icon: typeof Icon,
        name: string
        description: string
        button: {
            title: string
            icon: typeof Icon
            handler?: (() => void)
        }
    }

    const checkers: Record<keyof ColimaStatus, CheckerItem> = {
        isColimaInstalled: {
            icon: ServerCrash,
            name: 'Colima',
            description: "Is colima installed?",
            button: {
                title: "Install",
                icon: Download
            }
        },
        isDockerInstalled: {
            icon: Container,
            name: 'Docker',
            description: "Is docker installed?",
            button: {
                title: "Install",
                icon: Download
            }
        },
        isColimaRunning: {
            icon: ServerCog,
            name: 'Colima Status',
            description: "Is colima running?",
            button: {
                title: "Start",
                icon: Power,
                handler: startColima
            }
        },
    }

    async function runChecks() {
        await invalidateAll()
        if (!status.isDockerInstalled) {
            toast.error("Docker is not installed yet.")
        }

        if (!status.isColimaInstalled) {
            toast.error("Colima is not installed yet.")
        }

        if (!status.isColimaRunning) {
            toast.warning("Colima is not running yet.")
        }
    }

    async function startColima() {
        loadingState.isColimaRunning = true;
        await startColimaService()
        loadingState.isColimaRunning = false;
        await invalidateAll()
        if (!status.isColimaRunning) {
            toast.warning("Colima is not running yet.")
        } else {
            toast.success("Colima is running.")
        }
    }
</script>

<Card.Root class="w-[380px]">
    <Card.Header>
        <Card.Title>Service Status</Card.Title>
        <Card.Description>
            Required services to run `Etheria`.
        </Card.Description>
    </Card.Header>
    <Card.Content class="grid gap-4">
        {#each Object.keys(checkers) as checkerName, index (index)}
            {@const checker = checkers[checkerName]}
            {@const PrimaryIcon = checker.icon}
            {@const SecondaryIcon = checker.button.icon}
            <div class=" flex items-center space-x-4 rounded-md border p-4">
                <PrimaryIcon />
                <div class="flex-1 space-y-1">
                    <p class="text-sm font-medium leading-none">{checker.name}</p>
                    <p class="text-muted-foreground text-sm">
                        {checker.description}
                    </p>
                </div>
                {#if status[checkerName]}
                    <Switch checked={status[checkerName]} disabled/>
                    {:else }
                    <Button size="sm" onclick={checker.button.handler} disabled={checkerName === 'isColimaRunning' && (!status.isColimaInstalled || !status.isDockerInstalled || loadingState[checkerName])}>
                        {#if loadingState[checkerName]}
                            <LoaderCircle class="animate-spin" />
                        {:else}
                            <SecondaryIcon />
                        {/if}
                        {checker.button.title}
                    </Button>
                {/if}
            </div>
        {/each}
    </Card.Content>
    <Card.Footer>
        <Button class="w-full" onclick={runChecks}>
            <Check class="mr-2 size-4" /> All ready?
        </Button>
    </Card.Footer>
</Card.Root>