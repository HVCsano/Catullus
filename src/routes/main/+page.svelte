<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { invoke } from '@tauri-apps/api/core';
	import { goto } from '$app/navigation';

	let { data }: { data: PageData } = $props();

	onMount(async () => {
		let files = await invoke<string | number>('load_files_file');
		if (files === null) {
			await goto('/main/mismatch');
		}
	});
</script>

<div
	class=" -translate-1/2 absolute top-1/2 left-1/2 w-full text-center items-center justify-center z-20"
>
	<h1 class="font-tekt text-cat text-5xl font-bold animate-fade-down">Fájlok betöltése</h1>
	<h2 class="animate-fade-up animate-delay-200 mb-4">Elvileg ez nem tart annyira sokáig</h2>
</div>
