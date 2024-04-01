<script context="module">
	import DragImage from '$lib/DragImage.svelte';
	import PreviewImage from '$lib/PreviewImage.svelte';
	import NotificationHandler from '$lib/NotificationHandler.svelte';
</script>

<script lang="ts">
	let showOriginal: boolean = false;
	let showPreview: boolean = false;
	let loading: boolean = false;
	let smallerImageInput = false;
	$: imageUrl = '';
	$: croppedImageUrl = '';
	let latestResponse: BackendResponse;

	function scrollToBottom() {
		window.scrollTo({
			top: document.body.scrollHeight,
			behavior: 'smooth'
		});
	}

	function onFileSelected(newUrl: string) {
		const encoder = new TextEncoder();
		if (encoder.encode(newUrl).length > 524288) {
			latestResponse = {
				croppedImage: '',
				errorCode: 80
			};
			return;
		}

		imageUrl = newUrl;
		showOriginal = true;
		loading = false;
		showPreview = false;
		smallerImageInput = true;

		setTimeout(() => {
			scrollToBottom();
		}, 500);
	}

	async function eraseBackground() {
		loading = true;
		setTimeout(() => {
			scrollToBottom();
		}, 500);

		const response = await fetch('/api/erase', {
			method: 'POST',
			body: JSON.stringify(imageUrl),
			headers: {
				'content-type': 'application/json'
			}
		})
			.then((response: any) => response.json())
			.catch(() => {
				return { croppedImage: '', errorCode: 100 };
			});

		latestResponse = response;
		if (response.croppedImage !== '') {
			croppedImageUrl = response.croppedImage;
			showPreview = true;
		}

		loading = false;
		setTimeout(() => {
			scrollToBottom();
		}, 500);
	}

	function downloadImage() {
		const anchor = document.createElement('a');
		anchor.href = croppedImageUrl;
		anchor.download = 'cropped_image.png'; // Set the download filename
		document.body.appendChild(anchor);
		anchor.click();
		document.body.removeChild(anchor); // Clean up the anchor element
	}
</script>

<NotificationHandler answer={latestResponse} />

<div class="py-8 md:py-16 flex flex-col gap-4 items-center font-['Kanit']">
	<h1 class="text-3xl md:text-6xl font-bold">Background Eraser</h1>

	<section class="flex flex-col w-4/5 max-w-[900px] items-center gap-4">
		<p>Upload your image down below, press 'Erase' and wait for you new image to download!</p>

		<div class="flex flex-col gap-4 items-center justify-center w-full">
			<DragImage onFileChange={onFileSelected} small={smallerImageInput} />

			<div class="w-full flex flex-col md:flex-row md:justify-between">
				<div class="w-full md:w-[45%]">
					<PreviewImage
						{imageUrl}
						title={'Original:'}
						show={showOriginal}
						buttonText="Remove Background"
						onButtonClick={eraseBackground}
					/>
				</div>

				<div class="w-full md:w-[45%]">
					<PreviewImage
						{loading}
						imageUrl={croppedImageUrl}
						title={'Preview:'}
						buttonText="Download"
						onButtonClick={downloadImage}
						show={showPreview}
					/>
				</div>
			</div>
		</div>
	</section>
</div>
