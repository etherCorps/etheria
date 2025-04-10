<script lang="ts">
  import type {PageProps} from "./$types";
  import {pageState} from "$lib/stores.svelte";
  import {onMount} from "svelte";
  import FallbackOnError from "$lib/components/ui/running-services/fallback-on-error.svelte"

  let {data}: PageProps = $props();

  onMount(async () => {
    pageState.isLoading = false;
  })
</script>

<div class="flex flex-row items-center justify-center h-full">
    <FallbackOnError {...data}/>
</div>
