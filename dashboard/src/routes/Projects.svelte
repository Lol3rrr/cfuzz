<script lang="ts">
	import { onMount } from "svelte";
	import Collapsable from "../components/Collapsable.svelte";

	import * as api from "../api";
	import * as store from "../store";

	let projects: Array<api.Project> = [];
	store.projects.subscribe((data) => {
		projects = data;
	});

	onMount(() => {
		store.updateProjects();
	});

	let addProjectExpand = false;
	function expandAddProject() {
		addProjectExpand = !addProjectExpand;
	}

	let name: String = "";
	let repo: String = "";
	function addProject() {
		if (name.trim().length == 0 || repo.trim().length == 0) {
			return;
		}

		api.addProject(name.trim(), repo.trim());
	}
</script>

<div>
	<h1>Projects</h1>

	{#each projects as project}
		<a href={"#/project/" + project.name}>
			<h2>{project.name}</h2>
		</a>
	{/each}

	<div>
		<h3 on:click={expandAddProject}>Add Project</h3>
		<Collapsable bind:collapsed={addProjectExpand}>
			<div>
				<form>
					<label for="name">Name</label>
					<input
						type="text"
						name="name"
						id="name"
						bind:value={name}
					/>
					<label for="git">Git-Repo</label>
					<input type="text" name="git" id="git" bind:value={repo} />
					<input type="button" value="Add" on:click={addProject} />
				</form>
			</div>
		</Collapsable>
	</div>
</div>

<style>
	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}
</style>
