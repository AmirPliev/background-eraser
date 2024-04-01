<script context="module">
	import { Button } from '$lib/components/ui/button';
	import { Circle2 } from 'svelte-loading-spinners';
	import { scale } from 'svelte/transition';
</script>

<script lang="ts">
	export let imageUrl: string = '';
	export let title: string;
	export let show: boolean = false;
	export let buttonText: string;
	export let onButtonClick: () => void = () => {};
	export let loading: boolean = false;

	const transitionParams = {
		duration: 500,
		opacity: 0
	};
</script>

{#if loading}
	<div in:scale={transitionParams} class="flex py-16 w-full h-full justify-center items-center">
		<Circle2 size="60" />
	</div>
{:else if !loading && show}
	<div in:scale={transitionParams}>
		<h2 class="text-xl pb-4">{title}</h2>
		<div class="relative w-full fit-content rounded-xl overflow-hidden">
			<img alt="Some alternative" src={imageUrl} class="w-full object-contain" />

			<img
				alt="bg"
				src="https://thumbnail.imgbin.com/3/9/9/imgbin-transparent-snow-for-backgrounds-snowflakes-illustration-c8Yp8BTpQMmDbVuTWPTZ6vmn6_t.jpg"
				class="absolute top-0 left-0 -z-10 bg-red-500 w-full h-full"
			/>
		</div>

		<div class="flex justify-end w-full">
			<Button class="mt-4" on:click={onButtonClick}>{buttonText}</Button>
		</div>
	</div>
{/if}
