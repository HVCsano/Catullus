<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';
	import { getVersion } from '@tauri-apps/api/app';
	import { onMount } from 'svelte';
	import { emit, listen } from '@tauri-apps/api/event';
	import { goto } from '$app/navigation';

	listen<string>('changepanel', (ev) => {
		goto(ev.payload, { replaceState: true });
	});

	let ver = $state('');

	onMount(async () => {
		ver = await getVersion();
		await emit('panel');
	});

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

{@render children()}

<h2 class="text-gray-400 absolute bottom-0 left-1 z-20 pointer-events-none select-none">v{ver}</h2>
