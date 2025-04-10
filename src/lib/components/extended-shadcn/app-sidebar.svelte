<script lang="ts">
    import * as Sidebar from '$lib/components/shadcn/sidebar';
    import {
        Cat,
        Sun,
        Moon,
        StopCircle,
        type Icon, LoaderCircle, Layers, Package, Box
    } from '@lucide/svelte';
    import { mode, setMode } from 'mode-watcher';
    import { page } from '$app/state';
    import { useSidebar } from '$lib/components/shadcn/sidebar';
    import {afterNavigate, invalidateAll} from '$app/navigation';
    import type {ColimaStatus} from "$lib/invoker/installation";
    import {stopColimaService} from "$lib/invoker/colima";
    import {toast} from "svelte-sonner";

    type MenuSectionItem = {
        title: string;
        url: string;
        icon: typeof Icon;
    };

    const mainMenuSection: MenuSectionItem[] = [
        {
            title: 'Containers',
            url: '/containers',
            icon: Package
        },
        {
            title: 'Images',
            url: '/images',
            icon: Box
        },
        {
            title: 'Volumes',
            url: '/volumes',
            icon: Layers
        },
    ];

    type MenuSection = {
        title: string;
        menu: MenuSectionItem[];
    };

    const menuSections: MenuSection[] = [
        {
            title: 'Docker Manager',
            menu: mainMenuSection
        }
    ];

    function handleModeChange() {
        setMode($mode === 'light' ? 'dark' : 'light');
    }

    const sidebar = useSidebar();
    afterNavigate(async () => {
        if (sidebar.open) {
            sidebar.setOpenMobile(false);
        }
    });

    let { isDockerInstalled, isColimaInstalled, isColimaRunning } : ColimaStatus = $props();

    let isStoppingColima = $state(false);

    async function handleStopColima() {
        isStoppingColima = true;
        await stopColimaService();
        await invalidateAll();
        isStoppingColima = false;
        toast.warning('Colima has been stopped.');
    }
</script>

<Sidebar.Root collapsible="icon">
    <Sidebar.Header>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.MenuButton
                        class="hover:bg-transparent data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        size="lg"
                >
                    <div
                            class="flex aspect-square size-8 items-center justify-center rounded-lg text-primary"
                    >
                        <Cat class="stroke-primary" />
                    </div>

                    <span class="font-sour-gummy text-xl text-primary">Docker</span>
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Header>
    <Sidebar.Content>
        {#each menuSections as section (section.title)}
            <Sidebar.Group>
                <Sidebar.GroupLabel>{section.title}</Sidebar.GroupLabel>
                <Sidebar.GroupContent>
                    <Sidebar.Menu>
                        {#each section.menu as item (item.title)}
                            <Sidebar.MenuItem>
                                <Sidebar.MenuButton isActive={page.url.pathname === item.url}>
                                    {#snippet child({ props })}
                                        <a href={item.url} {...props}>
                                            <item.icon />
                                            <span>{item.title}</span>
                                        </a>
                                    {/snippet}
                                </Sidebar.MenuButton>
                            </Sidebar.MenuItem>
                        {/each}
                    </Sidebar.Menu>
                </Sidebar.GroupContent>
            </Sidebar.Group>
        {/each}
    </Sidebar.Content>
    <Sidebar.Footer>
        <Sidebar.Menu>
            {#if isColimaInstalled && isDockerInstalled && isColimaRunning}
                <Sidebar.MenuItem>
                    <Sidebar.MenuButton onclick={handleStopColima}>
                        {#if isStoppingColima}
                            <LoaderCircle class="animate-spin" />
                        {:else }
                            <StopCircle class="h-5 w-5 {isStoppingColima ? 'animate-spin' : ''}" />
                        {/if}
                        <span class="sr-only">Stop Colima</span>
                        <span>Stop Colima</span>
                    </Sidebar.MenuButton>
                </Sidebar.MenuItem>
            {/if}
            <Sidebar.MenuItem>
                <Sidebar.MenuButton onclick={handleModeChange}>
                    {#if $mode === 'light'}
                        <Moon class="h-5 w-5" />
                    {:else}
                        <Sun class="h-[1.5rem] w-[1.3rem]" />
                    {/if}
                    <span class="sr-only">Toggle theme</span>
                    <span>{$mode === 'light' ? 'Dark Mode' : 'Light Mode'}</span>
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Footer>
</Sidebar.Root>
