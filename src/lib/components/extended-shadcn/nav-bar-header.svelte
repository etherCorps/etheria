<script>
import {page} from "$app/state";
import * as Sidebar from "$lib/components/shadcn/sidebar/index.js";
import * as Breadcrumb from "$lib/components/shadcn/breadcrumb/index.js";
import {kebabToNormal} from "$lib/helpers/convertors.js";
import {Separator} from "$lib/components/shadcn/sidebar/index.js";
</script>

<header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
    <Sidebar.Trigger class="-ml-1" />
    <Separator orientation="vertical" class="mr-2 h-4" />
    <Breadcrumb.Root>
        <Breadcrumb.List>
            <Breadcrumb.Item class="hidden md:block">
                <Breadcrumb.Link href="#">Docker</Breadcrumb.Link>
            </Breadcrumb.Item>
            <Breadcrumb.Separator class="hidden md:block" />
            <Breadcrumb.Item>
                <Breadcrumb.Page>
                    {#if page.url.pathname === '/'}
                        Initialize
                    {:else}
                        {kebabToNormal(page.url.pathname)}
                    {/if}
                </Breadcrumb.Page>
            </Breadcrumb.Item>
        </Breadcrumb.List>
    </Breadcrumb.Root>
</header>