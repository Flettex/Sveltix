<script lang="ts">
    import type { PageData } from "./$types";
    import {counter} from "$lib/stores";
    export let data: PageData;
</script>

<h1>Message from server: {data.data}</h1>

<a href="/about">About page</a>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<div class="counter">
    <button on:click={() => counter.update(c => c - 1)}>-</button>
    <span>{$counter}</span>
    <button on:click={() => counter.update(c => c + 1)}>+</button>
</div>

{#if $counter > 3}
    <span>the count is very high</span>
{/if}