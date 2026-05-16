<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
</script>

<main class="bg-slate-700 absolute w-full h-full text-center text-white">
  <div
    class=" -translate-1/2 absolute top-1/2 left-1/2 w-full text-center items-center justify-center"
  >
    <h1 class="font-tekt text-cat text-5xl font-bold animate-fade-down">
      Üdvözöl a Catullus!
    </h1>
    <h2 class="animate-fade-up animate-delay-200">
      A gyors, egyszerű SeeMTA mod platform.
    </h2>
    <button
      class="bg-gren animate-wiggle animate-infinite mt-4 text-3xl font-tekt font-bold px-6 py-2 rounded-xl cursor-pointer hover:bg-gren-dark duration-200"
      >Kezdés</button
    >
  </div>
</main>
