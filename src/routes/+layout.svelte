<script lang="ts">
    import AppSidebar from "$lib/components/extended-shadcn/app-sidebar.svelte";
    import NavbarHeader from "$lib/components/extended-shadcn/nav-bar-header.svelte";
    import * as Sidebar from "$lib/components/shadcn/sidebar";
    import { Toaster } from "$lib/components/shadcn/sonner";
    import * as Card from "$lib/components/shadcn/card"
    import FallbackOnError from "$lib/components/ui/running-services/fallback-on-error.svelte"
    import { ModeWatcher } from 'mode-watcher';
    import '../app.css';
    import type {LayoutProps} from "./$types";
    let {data, children}: LayoutProps = $props()
</script>

<ModeWatcher defaultMode="light" />
<Toaster richColors closeButton/>

<Sidebar.Provider>
    <AppSidebar {...data} />
    <Sidebar.Inset>
        <NavbarHeader />
        <div class="flex flex-1 flex-col gap-4 p-4">
                {@render children()}
        </div>
    </Sidebar.Inset>
</Sidebar.Provider>
