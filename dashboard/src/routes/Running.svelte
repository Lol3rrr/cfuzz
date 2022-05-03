<script lang="ts">
    import { onMount } from "svelte";

    import { loadRunning } from "../api";

    let running: Array<String> = [];

    onMount(() => {
        loadRunning().then((data) => {
            running = data;
        });
    });
</script>

<div>
    <h2>Running</h2>
    {#each running as runner}
        <div class="running">
            <h3>{runner}</h3>
        </div>
    {/each}
</div>

<style>
    .running {
        background-color: #dddddd;
        border-radius: 5px;

        width: 60%;
        margin: auto;
        padding: 0.3rem;
    }
</style>
