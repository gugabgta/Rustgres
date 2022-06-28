<div class="wrapper row">
	<div class="sidebar">
		<SideMenu />
	</div>
	<div class="content">
		<Tabs />
		<FunctionsMenu
			on:play={ eventPlay }
			on:stop={ eventStop }
		/>
		<QueryZone />
		<Results bind:queryResults={ queryResults }/>
	</div>
</div>
<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import QueryZone, { textContent }  from './QueryZone.svelte'
	import SideMenu from './SideMenu.svelte'
	import FunctionsMenu from './FunctionsMenu.svelte';
	import Results from './Results.svelte';
	import Tabs from './Tabs.svelte';

	let queryResults = '';
	function eventPlay() {
		return invoke('execute', {query: $textContent}).then((res: string) =>
		{
			queryResults = res
		}, () => {
			queryResults = "Something went wrong"
		})
	}

	function eventStop() {
		console.log('stop was clicked')
	}
</script>

<style>
	.sidebar {
		height: 100vh;
        z-index: 999;
		width: 20%;
		min-width: 80px;
		max-width: 320px;
		top: 0px;
		left: 0px;
	}

	.content {
		height: 100vh;
		width: fit-content;
	}
</style>
