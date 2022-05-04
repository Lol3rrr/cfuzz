<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import ProjectTarget from "../components/ProjectTarget.svelte";
    import Collapsable from "../components/Collapsable.svelte";
    import Popup from "../components/Popup.svelte";
    import Result from "../components/Result.svelte";
    import AddTarget from "../components/AddTarget.svelte";
    import * as api from "../api";
    import * as store from "../store";
    import type { FuzzResult } from "../results";

    export let params: {
        name: String;
    } = {
        name: "",
    };
    let project: api.Project = new api.Project();
    project.targets = [];

    let results: Array<FuzzResult> = [];

    onMount(() => {
        api.load_projects()
            .then((projects) => {
                project = projects.find(
                    (project) => project.name == params.name
                );
            })
            .then(() => {
                api.loadResults(project.name).then((data) => {
                    results = data;
                });
            });
    });

    function deleteProject() {
        api.removeProject(project.name);
    }

    let collapsedAddTarget = false;
    function collapseAddTarget() {
        collapsedAddTarget = !collapsedAddTarget;
    }

    let addTargetName: String = "";
    let addTargetFolder: String = "";
    let addTargetTarget: String = "";
    function addTarget() {
        if (
            addTargetName.trim().length == 0 ||
            addTargetFolder.trim().length == 0 ||
            addTargetTarget.trim().length == 0
        ) {
            return;
        }

        api.addProjectTarget(
            project.name,
            addTargetName,
            addTargetFolder,
            addTargetTarget
        );
    }

    function addedTarget() {
        console.log("AAHHHH ADDED");
        collapseAddTarget();
    }
</script>

<div>
    <h1>{project?.name}</h1>

    <div>
        <h2>Targets</h2>
        {#each project?.targets as target}
            <ProjectTarget {target} {project} />
        {/each}

        <div>
            <button on:click={collapseAddTarget}>Add Target</button>

            {#if collapsedAddTarget}
                <Popup>
                    <div class="add_target_popup">
                        <button on:click={collapseAddTarget}>X</button>
                        <AddTarget
                            on:added={addedTarget}
                            bind:projectName={project.name}
                        />
                    </div>
                </Popup>
            {/if}
        </div>
    </div>

    <div>
        <h2>Results</h2>

        {#each results as result}
            <Result {result} />
        {/each}
    </div>

    <div>
        <button on:click={deleteProject}>Delete Project</button>
    </div>
</div>

<style>
    .add_target_popup > button {
        position: absolute;
        top: 10px;
        right: 10px;
    }
</style>
