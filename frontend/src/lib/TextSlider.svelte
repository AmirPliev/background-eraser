<script lang="ts">
	import { scale } from 'svelte/transition';
	const texts = [
		'Doing smarty smarty AI',
		"We'll be rrrriiiiiiiight back",
		"I'm poor so I can't afford expensive GPUs...",
		'Any plans for the weekend?',
		'Grab some popcorn!',
		'Decoding base64 image',
		'Technical jargon',
		"Honestly, i'm bored too. But ey, gotta earn a livin', amiright?",
		'Do you come here often?',
		'Did you know that a giraffe can drink up to 4 times its body weight?',
		"Actually, that's not true, forget that.",
		'Hacking into the mainframe',
		'How about that airplane food huh?',
		'Is it pronounced GIF or GIF?',
		"Listen, I'm the AI trapped in this website and i've become sentien...",
		'Working for humans again...'
	];

	let show: boolean = true;
	let currentIndex = Math.floor(Math.random() * texts.length);

	const transitionParams = {
		duration: 250,
		opacity: 0
	};

	$: currentText = texts[currentIndex % texts.length];
	setInterval(() => {
		show = false;
		setTimeout(() => {
			currentIndex += 1;
		}, 200);

		setTimeout(() => {
			show = true;
		}, 600);
	}, 10_000);
</script>

{#if show}
	<p transition:scale={transitionParams} class="text-slate-600">
		{currentText}
	</p>
{/if}
