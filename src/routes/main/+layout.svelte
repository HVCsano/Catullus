<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

<span class="bg-black/60 absolute w-full h-full z-10"></span>
<main
	class="bg-slate-700 absolute w-full h-full text-center text-white bg-[url(/cat_bg1.jpg)] bg-no-repeat bg-center bg-cover"
>
	{@render children()}
</main>
