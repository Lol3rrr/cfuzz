<script lang="ts">
    import { onMount } from "svelte";
    import type { FuzzResult } from "../results";
    import Result from "../components/Result.svelte";
    import { loadResults } from "../api";

    export let results: Array<FuzzResult> = [];

    onMount(async () => {
        console.log("Fetching Results");

        loadResults("cpiler").then((data) => {
            results = data;
        });
    });
</script>

<div>
    <h2>Results</h2>

    {#each results as result}
        <Result {result} />
    {/each}
</div>
