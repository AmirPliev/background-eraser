<script lang="ts">
	export let onFileChange: (newUrl: string) => void;
	export let small: boolean = false;
	$: heightStyle = small ? 'h-32' : 'h-64';

	function onChange(event: Event) {
		const target = event.target as HTMLInputElement;

		if (target === undefined) {
			return;
		}

		const selectedFiles = target.files;
		if (!selectedFiles) {
			return;
		}

		const selectedFile = selectedFiles[0];
		var reader = new FileReader();
		reader.onload = function (event) {
			if (!event.target) {
				return;
			}

			onFileChange(event.target.result as string);
		};

		reader.readAsDataURL(selectedFile);
	}
</script>

<label
	for="dropzone-file"
	class={`flex flex-col items-center transition-all duration-1000 justify-center 
          w-full ${heightStyle} border-2 border-gray-300 border-dashed rounded-lg 
          cursor-pointer bg-gray-50`}
>
	<div class="flex flex-col items-center justify-center pt-5 pb-6">
		<svg
			class="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400"
			aria-hidden="true"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 20 16"
		>
			<path
				stroke="currentColor"
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
			/>
		</svg>

		<p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
			<span class="font-semibold">Click to upload</span>
		</p>

		<p class="text-xs text-gray-500 dark:text-gray-400">SVG, PNG, JPG (MAX: 20MB)</p>
	</div>

	<input
		on:change={onChange}
		name="originalImage"
		id="dropzone-file"
		type="file"
		required
		accept=".jpg, .jpeg, .png"
		class="hidden"
	/>
</label>
