<script lang="ts">
	import { getVersion } from '@tauri-apps/api/app';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let ver = $state('');

	listen<string>('selectedGameDir', (ev) => {
		seefolder = ev.payload;
	});

	let page = $state(1);

	let seefolder = $state('C:\\SeeMTA');

	async function setGameDir() {
		await invoke('set_game_dir');
	}
	async function restart() {
		await invoke('done_setup');
	}

	async function saveDir() {
		await invoke('save_game_dir', { dir: seefolder });
		page = 4;
	}

	onMount(async () => {
		ver = await getVersion();
	});
</script>

<span class="bg-black/60 absolute w-full h-full z-10"></span>
<main
	class="bg-slate-700 absolute w-full h-full text-center text-white bg-[url(/cat_bg1.jpg)] bg-no-repeat bg-center bg-cover"
>
	{#if page === 1}
		<div
			class=" -translate-1/2 absolute top-1/2 left-1/2 w-full text-center items-center justify-center z-20"
		>
			<h1 class="font-tekt text-cat text-5xl font-bold animate-fade-down">Üdvözöl a Catullus!</h1>
			<h2 class="animate-fade-up animate-delay-200 mb-4">A gyors, egyszerű SeeMTA mod platform.</h2>
			<button
				onclick={() => (page = 2)}
				class="bg-gren animate-wiggle animate-infinite text-3xl font-tekt font-bold px-6 py-2 rounded-xl cursor-pointer hover:bg-gren-dark duration-200"
				>Kezdés</button
			>
		</div>
	{/if}
	{#if page !== 1}
		<div class="-translate-x-1/2 fixed top-4 left-1/2 w-full z-20">
			<h1 class="font-tekt text-cat text-5xl font-bold animate-fade-down">
				Catullus telepítési mágus
			</h1>
		</div>
	{/if}
	{#if page === 2}
		<div
			class=" -translate-1/2 absolute top-1/2 left-1/2 w-[80vw] text-center items-center justify-center z-20 animate-fade-up"
		>
			<h3 class="text-cat font-bold animate-pulse text-3xl mb-2">Hogyan működik a Catullus?</h3>
			<p>
				A program felfedezi az aktuális (eredeti) fájlokat, így a modolásnál pontosan látni éppen
				mely verzió van betöltve a játék által, amely egy kattintással módosítható.
			</p>
			<button
				onclick={() => (page = 3)}
				class="bg-gren animate-wiggle animate-infinite text-3xl font-tekt font-bold px-6 py-2 rounded-xl cursor-pointer hover:bg-gren-dark duration-200 mt-2"
				>Tovább</button
			>
		</div>
	{/if}
	{#if page === 3}
		<div
			class=" -translate-1/2 absolute top-1/2 left-1/2 w-[80vw] text-center items-center justify-center z-20 flex flex-col animate-fade-up"
		>
			<h3 class="text-cat font-bold animate-pulse text-3xl mb-2">SeeMTA telepítési mappája</h3>
			<p>Kérlek add meg a SeeMTA mappáját!</p>
			<h1 class="font-bold text-xl font-mono">Aktuális mappa: {seefolder}</h1>
			<button
				onclick={() => setGameDir()}
				class="bg-amber-300 font-bold text-black rounded-xl px-2 py-1 cursor-pointer hover:bg-amber-600 duration-200 transition-colors"
				>Nekem máshol van, mutatom</button
			>
			<button
				onclick={async () => await saveDir()}
				class="bg-gren animate-wiggle animate-infinite text-3xl font-tekt font-bold px-6 py-2 rounded-xl cursor-pointer hover:bg-gren-dark duration-200 mt-2"
				>Tovább</button
			>
		</div>
	{/if}
	{#if page === 4}
		<div
			class=" -translate-1/2 absolute top-1/2 left-1/2 w-[80vw] text-center items-center justify-center z-20 flex flex-col animate-fade-up"
		>
			<h3 class="text-green-400 font-bold animate-pulse text-3xl mb-2">Meg is vagyunk!</h3>
			<p>Remélem nem vettem el sok időt az életedből, meg hogy tetszik a háttér. 😜</p>
			<p>Innen már más dolgot nincs, mint megfejelni az alsó gombot és bent is vagy!</p>
			<button
				onclick={async () => await restart()}
				class="bg-gren animate-wiggle animate-infinite text-3xl font-tekt font-bold px-6 py-2 rounded-xl cursor-pointer hover:bg-gren-dark duration-200 mt-2"
				>Megfejelem az alsó gombot</button
			>
		</div>
	{/if}
</main>
