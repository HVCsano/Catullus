<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageProps } from './$types';
	import { emit, listen } from '@tauri-apps/api/event';

	let { data }: PageProps = $props();

	onMount(async () => {
		await emit('progress');
	});

	listen<string>('setprogress-title', (ev) => {
		title = ev.payload;
	});
	listen<string>('setprogress-desc', (ev) => {
		desc = ev.payload;
	});
	listen<string>('setprogress-undesc', (ev) => {
		undesc = ev.payload;
	});
	listen<number>('setprogress-state', (ev) => {
		console.log(ev.payload);
		state = ev.payload;
	});

	let title = $state('');
	let desc = $state('');
	let undesc = $state('');
	let state: number = $state(0);
</script>

<div
	class=" -translate-1/2 absolute top-1/2 left-1/2 w-full text-center items-center justify-center z-20 flex-col flex"
>
	<h1>{title}</h1>
	<h2>{desc}</h2>
	<progress max="100" value={state}>{state}%</progress>
	<h2>{undesc}</h2>
</div>
