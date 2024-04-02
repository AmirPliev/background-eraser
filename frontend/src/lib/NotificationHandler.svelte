<script context="module">
	import { Toaster } from '$lib/components/ui/sonner';
	import { toast } from 'svelte-sonner';
</script>

<script lang="ts">
	const codeToMessage: { [x: number]: string } = {
		0: 'Processing your image failed. Is the image type correct?',
		1: "Couldn't find any objects in your image, is something even visible?",
		2: 'Processing your image failed. Is the image type correct?',
		80: 'Your images is too large :(, Make sure to check the max resolution limit.',
		90: 'Backend Side: Something seems to be wrong sadly. Try coming back again later.',
		100: 'Server Side: Something seems to be wrong sadly. Try coming back again later.'
	};

	export let answer: BackendResponse;

	$: {
		if (answer) {
			if (answer.errorCode !== 0) {
				toast.error(codeToMessage[answer.errorCode]);
			}
		}
	}
</script>

<Toaster richColors position="bottom-right" expand={true} />
