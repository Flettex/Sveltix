<script lang="ts">
    export let start_count: Number = 1;
</script>

<div class="counter">
    <button on:click={() => start_count = start_count - 1}>-</button>
    <span>{start_count}</span>
    <button on:click={() => start_count = start_count + 1}>+</button>
</div>

<style>
    .counter {
        display: flex;
        gap: 0.5rem;
    }
</style>