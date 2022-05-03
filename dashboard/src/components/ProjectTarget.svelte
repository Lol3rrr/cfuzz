<script lang="ts">
    import * as api from "../api";
    import Collapsable from "./Collapsable.svelte";

    let collapsed = true;

    export let project: api.Project;
    export let target: api.ProjectTarget;

    function expandContent() {
        collapsed = !collapsed;
    }

    function runTarget() {
        api.run(project.name, target.name);
    }
</script>

<div class="target">
    <h3>{target.name}</h3>

    <button class="runButton" on:click={runTarget}>Run</button>

    <div class="expandbutton-background" on:click={expandContent}>
        <i class="expandbutton" class:collapsed />
    </div>

    <Collapsable bind:collapsed>
        <p>Folder: {target.folder}</p>
        <p>Target: {target.target.CargoFuzz.name}</p>
    </Collapsable>
</div>

<style>
    .target {
        position: relative;

        background-color: #cccccc;
        border-radius: 0.5rem;
        width: 75%;
        margin: 10px auto;
        padding: 0.1rem;
    }

    .runButton {
        position: absolute;
        right: 5em;
        top: 1em;
        border-radius: 5px;

        background-color: #22ee22;

        font-weight: 500;
    }

    .expandbutton-background {
        position: absolute;
        right: 2em;
        top: 1em;

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
