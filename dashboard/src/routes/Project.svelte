<script lang="ts">
    import { onMount } from "svelte";
    import ProjectTarget from "../components/ProjectTarget.svelte";
    import Collapsable from "../components/Collapsable.svelte";
    import Result from "../components/Result.svelte";
    import * as api from "../api";
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
                console.log(project);
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

            <Collapsable bind:collapsed={collapsedAddTarget}>
                <div>
                    <form>
                        <label for="target_name">Target-Name</label>
                        <input
                            type="text"
                            name="target_name"
                            id="target_name"
                            bind:value={addTargetName}
                        />
                        <label for="target_folder">Target-Folder</label>
                        <input
                            type="text"
                            name="target_folder"
                            id="target_folder"
                            bind:value={addTargetFolder}
                        />
                        <label for="target_target">Fuzz-Target</label>
                        <input
                            type="text"
                            name="target_target"
                            id="target_target"
                            bind:value={addTargetTarget}
                        />
                        <button on:click={addTarget}>Add Target</button>
                    </form>
                </div>
            </Collapsable>
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
