<script lang="ts">
	import Router from "svelte-spa-router";
	import routes from "./routes";
	import Navbar from "./components/Navbar.svelte";
	import { onMount } from "svelte";

	import * as store from "./store";

	onMount(() => {
		store.updateProjects();
	});
</script>

<main>
	<Navbar />
	<div class="content">
		<Router {routes} />
	</div>
</main>

<style>
	main {
		text-align: center;

		display: grid;
		grid-template-columns: 20vw 80vw;
		grid-template-rows: 99vh;
	}

	.content {
		height: 100%;
		overflow: auto;
	}
</style>
