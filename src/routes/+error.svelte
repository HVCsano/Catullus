<script lang="ts">
	import { page } from '$app/state';
	import { getVersion } from '@tauri-apps/api/app';
	import { onMount } from 'svelte';

	let ver = $state('');

	onMount(async () => {
		ver = await getVersion();
	});
</script>

<span class="bg-black/60 absolute w-full h-full z-10"></span>
<main
	class="bg-slate-700 absolute w-full h-full text-center text-white bg-[url(/cat_bg1.jpg)] bg-no-repeat bg-center bg-cover pointer-events-none select-none"
>
	<div
		class=" -translate-1/2 absolute top-1/2 left-1/2 w-full text-center items-center justify-center z-20"
	>
		<h1 class="font-tekt text-red-500 text-5xl font-bold animate-fade-down">Bakfitty</h1>
		<h2 class="animate-fade-up animate-delay-200">Na ezt az oldalt nem kéne látnod 😒</h2>
		<h2 class="animate-fade-up animate-delay-200 mb-4">
			Hibakód: {page.status}
			{page.error!.message}
		</h2>
	</div>
</main>
