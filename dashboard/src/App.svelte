<script lang="ts">
	import Router from "svelte-spa-router";
	import routes from "./routes";
	import Navbar from "./components/Navbar.svelte";
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
		grid-template-columns: 15vw 85vw;
		grid-template-rows: 99vh;
	}

	.content {
		height: 100%;
		overflow: auto;
	}
</style>
