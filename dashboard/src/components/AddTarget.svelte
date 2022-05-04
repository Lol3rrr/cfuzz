<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import * as api from "../api";

    export let projectName: String;

    const dispatch = createEventDispatcher();

    let name = "";
    let folder = "";
    let repeating = false;
    let selected_type = "cargofuzz";
    let fuzz_target = "";

    function add() {
        name = name.trim();
        folder = folder.trim();
        fuzz_target = fuzz_target.trim();

        if (name.length == 0 || folder.length == 0) {
            console.log({ name, folder, repeating, fuzz_target });
            return;
        }

        if (selected_type == "cargofuzz") {
            if (fuzz_target.length == 0) {
                console.log("empty target name");
                return;
            }

            api.addProjectTarget(projectName, name, folder, fuzz_target).then(
                () => {
                    dispatch("added", {});
                }
            );
        }
    }
</script>

<div class="add_target_container">
    <h2>Add Target</h2>
    <div>
        <label for="name">Name</label>
        <input type="text" id="name" bind:value={name} />
        <label for="folder">Folder</label>
        <input type="text" id="folder" bind:value={folder} />
        <label for="repeating">Repeating</label>
        <input type="checkbox" id="repeating" bind:value={repeating} />
    </div>

    <select bind:value={selected_type}>
        <option value="cargofuzz">Cargo Fuzz</option>
        <option value="other">Other</option>
    </select>

    {#if selected_type == "cargofuzz"}
        <div>
            <h3>CargoFuzz</h3>
            <label for="fuzz_target">Fuzz-Target</label>
            <input type="text" id="fuzz_target" bind:value={fuzz_target} />
        </div>
    {/if}

    <button on:click={add}>Add</button>
</div>

<style>
    .add_target_container > h2 {
        color: #121212;
    }
</style>
