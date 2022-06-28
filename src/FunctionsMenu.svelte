<div class="wrapper">
    <button class="btn btn-danger" on:click="{ eventStop }">
        <svg><path d="{mdiStop}"/></svg>
    </button>
    <button class="btn btn-warning">
        <svg><path d="{mdiMagnify}"/></svg>
    </button>
    <button class="btn btn-secondary">
        <svg><path d="{mdiCogOutline}"/></svg>
    </button>
    <button class="btn btn-success" on:click="{ eventPlay }">
        <svg><path d="{mdiPlay}"/></svg>
    </button>
    <button class="btn btn-info">
        <svg><path d="{mdiDownload}"/></svg>
    </button>
</div>

<script lang="ts">
    import { mdiPlay, mdiCogOutline, mdiMagnify, mdiStop, mdiDownload } from '@mdi/js';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    function eventPlay() {
        dispatch('play')
    }

    function eventStop() {
        dispatch('stop')
    }
</script>

<style>
    .wrapper {
        margin-bottom: 5px;
    }

    svg {
        height: 40px;
        width: 40px;
    }

    button {
        height: 40px;
        width: 50px;
        border-radius: 5px;
    }
</style>
