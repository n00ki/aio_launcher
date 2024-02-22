<script lang="ts">
	// Utils
	import { onMount } from 'svelte';
	import {
		getOperatingSystem,
		launchLV1,
		launchWavesCentral,
		shutdownComputer
	} from '$lib/commands';

	// Assets
	import wavesLogo from '$lib/assets/waves.png';
	import ShutDownIcon from '$lib/components/ShutDownIcon.svelte';

	let isSystemSupported: boolean = true;

	onMount(async () => {
		const os = await getOperatingSystem();
		if (os !== 'Windows') {
			alert('This app is not supported on your operating system');
			isSystemSupported = false;
		}
	});
</script>

<div class="min-h-screen bg-slate-950 text-slate-50 flex flex-col justify-center items-center">
	<div class="flex justify-center items-center">
		<img src={wavesLogo} alt="Waves Audio" class="w-24 h-24" />
	</div>
	{#if isSystemSupported}
		<div class="pt-4 flex flex-wrap gap-4">
			<button
				on:click={launchLV1}
				class="h-28 w-28 border-4 border-orange-600 text-slate-50 px-4 py-2 rounded-full hover:bg-slate-900 transition-colors duration-300 ease-in-out"
				>LV1</button
			>
			<button
				on:click={launchWavesCentral}
				class="h-28 w-28 border-4 border-orange-600 text-slate-50 px-4 py-2 rounded-full hover:bg-slate-900 transition-colors duration-300 ease-in-out"
				>Waves Central</button
			>
		</div>
	{:else}
		<div>
			<h1 class="font-black text-3xl tracking-wide uppercase">
				your operating system is not supported
			</h1>
		</div>
	{/if}

	<footer class="absolute bottom-5 left-5">
		<p class="text-center text-xs">© Waves Live QA</p>
	</footer>
</div>

<div class="absolute right-5 bottom-5">
	<button
		on:click={shutdownComputer}
		class="rounded-full text-slate-200 p-2 hover:text-slate-50 hover:bg-slate-600 transition-all duration-300 ease-in-out"
	>
		<ShutDownIcon />
	</button>
</div>
