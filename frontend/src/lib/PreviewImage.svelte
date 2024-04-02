<script context="module">
	import { Button } from '$lib/components/ui/button';
	import { Circle2 } from 'svelte-loading-spinners';
	import TextSlider from './TextSlider.svelte';
	import { scale, fade } from 'svelte/transition';
	import { Confetti } from 'svelte-confetti';
</script>

<script lang="ts">
	export let imageUrl: string = '';
	export let oldImageUrl: string = '';
	export let title: string;
	export let show: boolean = false;
	export let buttonText: string;
	export let onButtonClick: () => void = () => {};
	export let loading: boolean = false;

	let removeOld: boolean = false;
	let showTada: boolean = false;
	let showConfetti: boolean = false;

	$: {
		if (!loading && show) {
			setTimeout(() => {
				removeOld = true;
			}, 1_000);
		}
	}

	const transitionParams = {
		duration: 500,
		opacity: 0
	};
</script>

{#if loading}
	<div
		in:scale={transitionParams}
		class="flex flex-col gap-4 py-16 w-full h-full justify-center items-center"
	>
		<div class="relative">
			<Circle2 size="60" />
			<p class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 font-bold">AI</p>
		</div>

		<div class="h-8">
			<TextSlider />
		</div>
	</div>
{:else if !loading && show}
	<div in:scale={transitionParams}>
		<h2 class="text-xl pb-4">{title}</h2>

		<div class="relative w-full fit-content rounded-xl overflow-hidden">
			<img
				alt="bg"
				src="https://thumbnail.imgbin.com/3/9/9/imgbin-transparent-snow-for-backgrounds-snowflakes-illustration-c8Yp8BTpQMmDbVuTWPTZ6vmn6_t.jpg"
				class="absolute top-0 left-0 -z-20 bg-red-500 w-full h-full"
			/>

			{#if oldImageUrl && !removeOld}
				<img
					alt="Some alternative"
					out:fade={{ duration: 1_000 }}
					on:outroend={() => {
						showConfetti = true;
						showTada = true;
					}}
					src={oldImageUrl}
					class="absolute top-0 left-0 -z-10 w-full object-contain"
				/>
			{/if}

			<img alt="Some alternative" src={imageUrl} class="w-full object-contain" />

			{#if showTada}
				<div class="absolute top-full left-1/2">
					<Confetti cone x={[-0.5, 0.5]} />
					<Confetti cone amount={10} x={[-1, -0.4]} y={[0.25, 0.75]} />
					<Confetti cone amount={10} x={[0.4, 1]} y={[0.25, 0.75]} />
				</div>
				<span transition:fade class="absolute top-0 left-0 w-full h-full bg-white/20" />

				<p
					class="absolute top-[10%] left-[10%] -rotate-[30deg] z-10"
					transition:scale={transitionParams}
					on:introend={() => {
						setTimeout(() => {
							showTada = false;
						}, 1_000);
					}}
				>
					Ta Da!
				</p>
			{/if}
		</div>

		<div class="flex justify-end w-full">
			<Button class="mt-4" on:click={onButtonClick}>{buttonText}</Button>
		</div>
	</div>
{/if}
