<div class="wrapper">
	<div class="sidebar">
		<SideBar />
	</div>
	<div class="content">
		<Tabs />
		<FunctionsMenu
			on:play={ eventPlay }
			on:stop={ eventStop }
		/>
		<QueryZone />
		<Results bind:queryResults={ queryResults } bind:update={ update }/>
	</div>
</div>
<!-- <svelte:window on:resize={onResize} /> -->
<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import QueryZone, { textContent }  from './QueryZone.svelte'
	import SideBar from './SideBar.svelte'
	import FunctionsMenu from './FunctionsMenu.svelte';
	import Results from './Results.svelte';
	import Tabs from './Tabs.svelte';

	let update: any
	let queryResults = '';

	function eventPlay() {
		return invoke('execute', {query: $textContent}).then((res: string) =>
		{
			queryResults = res
			update()
		}, () => {
			queryResults = "Something went wrong"
		})
	}

	function eventStop() {
		console.log('stop was clicked')
	}
</script>

<style>
	.wrapper {
		display: flex;
		flex-wrap: wrap;
		width: 100vw;
		background-color: rgb(46, 51, 56);
	}

	.sidebar {
		height: 100vh;
        z-index: 999;
		width: 25%;
		min-width: 200px;
		max-width: 400px;
		top: 0px;
		left: 0px;
		padding-right: -120px;
	}

	.content {
		height: 100vh;
		width: 75%;
	}
</style>
