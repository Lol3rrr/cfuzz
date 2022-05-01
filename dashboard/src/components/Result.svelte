<script lang="ts">
    import { onMount } from "svelte";

    import type { FuzzResult } from "../results";

    export let result: FuzzResult;

    import Collapsable from "./Collapsable.svelte";

    let collapsed = true;

    let download_button;

    function expandContent() {
        collapsed = !collapsed;
    }

    onMount(() => {
        let data = new Uint8Array(result.content);
        var file = new Blob([data], { type: "octet/stream" });
        download_button.href = URL.createObjectURL(file);
        download_button.download = "crash";
    });
</script>

<div class="result">
    <h3>{result.name}</h3>
    <a href="" bind:this={download_button}>Download Crash File</a>
    <div class="expandbutton-background" on:click={expandContent}>
        <i class="expandbutton" class:collapsed />
    </div>
    <Collapsable bind:collapsed>
        <p>
            {result.content.join(", ")}
        </p>
    </Collapsable>
</div>

<style>
    .result {
        position: relative;
        border-radius: 10px;
        background-color: #dedede;
        padding: 10px 10px;
        margin: 10px auto;
        width: 80%;
    }

    .expandbutton-background {
        position: absolute;
        right: 5em;
        top: 2em;

        background-color: #aeaeae;
        border-radius: 50%;
        padding-top: 0.4rem;
        height: 1.6rem;
        width: 2rem;
    }
    .expandbutton {
        border: solid black;
        border-width: 0 3px 3px 0;
        display: inline-block;
        padding: 3px;

        transition: transform ease-in-out 0.4s;
    }
    .expandbutton:not(.collapsed) {
        transform: rotate(225deg);
    }
    .collapsed.collapsed {
        transform: rotate(45deg);
    }
</style>
