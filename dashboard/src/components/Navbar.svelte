<script lang="ts">
    import type { Project } from "../api";

    import * as store from "../store";

    import Collapsable from "./Collapsable.svelte";

    let projects: Array<Project> = [];
    store.projects.subscribe((data) => {
        projects = data;
    });

    let projectButton;
    let collapsed = true;
    function expandProjects() {
        console.log("Expand");
        collapsed = !collapsed;

        if (collapsed) {
            projectButton.textContent = "+";
        } else {
            projectButton.textContent = "-";
        }
    }
</script>

<div class="navbar">
    <div class="entry">
        <a href="#/">Projects</a>
        <button bind:this={projectButton} on:click={expandProjects}>+</button>

        <Collapsable {collapsed}>
            {#each projects as project}
                <a class="subproject" href={"#/project/" + project.name}
                    >{project.name}</a
                >
            {/each}
        </Collapsable>
    </div>
    <div class="entry">
        <a href="#/running">Running</a>
    </div>
    <div class="entry">
        <a href="#/results">Results</a>
    </div>
</div>

<style>
    .navbar {
        display: inline-block;
        width: 10rem;
        height: 99%;
        background-color: grey;

        display: flexbox;
    }

    a {
        color: #eeeeee;
        margin-left: 0.3rem;
        font-size: 1.5rem;
        padding: 0.2rem 0.5rem;
    }

    .entry {
        display: grid;
        grid-template-columns: 8rem 1.5rem;
        text-align: left;
    }

    .subproject {
        font-size: 1.3em;
        margin-left: 1.5rem;
    }

    button {
        display: inline-block;
        height: 1.7rem;
        width: 1.7rem;

        padding: 0px;

        margin-top: 0.3rem;
    }
</style>
