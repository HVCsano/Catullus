<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import type { PageData } from './$types';

	async function generate_files_file() {
		await invoke('generate_files_file');
	}

	let { data }: { data: PageData } = $props();
</script>

<div
	class=" -translate-1/2 absolute top-1/2 left-1/2 w-full text-center items-center justify-center z-20 flex-col flex"
>
	<h1 class="font-tekt text-red-500 text-5xl font-bold animate-fade-down">Fájlváltozás</h1>
	<p class="animate-fade-up animate-delay-200">
		Legutóbbi indítás óta változtak a fájlok, ez lehet egy SeeMTA frissítés vagy külső módosítás
		miatt.
	</p>
	<p class="animate-fade-up animate-delay-200">
		Első indításnál meg persze nem kell megijedni, mert nincsenek még eltárolva a fájlok, így mindig
		ide kerülsz ebben az esetben.
	</p>
	<button
		onclick={async () => await generate_files_file()}
		class="bg-orange-400 animate-fade-up animate-delay-300 text-3xl font-tekt mt-8 font-bold px-6 py-2 rounded-xl cursor-pointer hover:bg-orange-700 duration-200"
		>Fájlok újragenerálása</button
	>
	<p class="animate-fade-up animate-delay-400">
		Ezt válaszd első indításnál, illetve ha biztosan tudod, hogy nem szerkesztettél bele a
		játékfájlokba.
	</p>
	<button
		class="bg-blue-500 animate-fade-up animate-delay-300 text-3xl font-tekt mt-4 font-bold px-6 py-2 rounded-xl cursor-pointer hover:bg-blue-700 duration-200"
		>Catullussal modoltam csak a játékot</button
	>
	<p class="animate-fade-up animate-delay-400">
		Ezt válaszd ha a Catullust a legutóbbi modolás óta bezártad.
	</p>
</div>
