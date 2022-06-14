<div class="">
	<div class="row row-cols-2">
		<div id="side-menu" class="col">
			<SideMenu />
		</div>
		<div id="textarea" class="col">
			<QueryZone />
			<Results />
		</div>
	</div>
</div>

<script lang="ts">
	/* export let name: string; */
	import { invoke } from '@tauri-apps/api/tauri'
	import QueryZone from './QueryZone.svelte'
	import SideMenu from './SideMenu.svelte'
	import Results from './Results.svelte'

	let value = 0
	function changeValue() {
		invoke('randInt').then(function (res: number) {
			value = Math.floor(20 * res)+1
		})
	}
</script>

<style>
	#side-menu {
		width: 30%;
		height: 600px;
	}

	#textarea {
		width: 60%;
		height: 60%;
	}
</style>