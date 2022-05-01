<script lang="ts">
    import * as api from "../api";

    let name: String = "";
    let fuzz_target: String = "";
    let repo: String = "";
    let folder: String = "";

    function run() {
        if (
            name.trim().length == 0 &&
            fuzz_target.trim().length == 0 &&
            repo.trim().length == 0 &&
            folder.trim().length == 0
        ) {
            console.log(name, fuzz_target, repo, folder);
            return;
        }

        api.run(name.trim(), fuzz_target.trim(), repo.trim(), folder.trim());
    }
</script>

<div>
    <h2>Run</h2>

    <form>
        <label for="name">Name:</label>
        <input type="text" name="name" id="name" bind:value={name} />
        <label for="fuzz_target">Fuzz-Target</label>
        <input
            type="text"
            name="fuzz_target"
            id="fuzz_target"
            bind:value={fuzz_target}
        />
        <label for="repo">Repo</label>
        <input type="text" name="repo" id="repo" bind:value={repo} />
        <label for="folder">Folder</label>
        <input type="text" name="folder" id="folder" bind:value={folder} />
        <input type="button" value="Run" on:click={run} />
    </form>
</div>
