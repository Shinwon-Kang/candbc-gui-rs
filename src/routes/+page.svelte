<script>
	import { Indicator } from 'flowbite-svelte';
	import Dropzone from 'svelte-file-dropzone';
	import { UploadOutline, FileOutline, AngleRightOutline } from 'flowbite-svelte-icons';

	let files = {
		accepted: [],
		rejected: []
	};

	function handleFilesSelect(e) {
		const { acceptedFiles, fileRejections } = e.detail;
		files.accepted = [...files.accepted, ...acceptedFiles];
		files.rejected = [...files.rejected, ...fileRejections];
	}
</script>

<!-- containerClasses="h-3/4 w-2/3 place-self-center place-content-center" -->

<div class="mx-48 flex flex-1">
	<Dropzone
		on:drop={handleFilesSelect}
		containerClasses="text-gray-400 bg-gray-100 h-3/4 w-2/3 border-dashed border-indigo-50 border-2 rounded-2xl place-self-center place-content-center place-items-center dark:bg-gray-800 grid"
		disableDefaultStyles
		accept=".dbc"
	>
		<UploadOutline class="w-10 h-10 mb-3 dark:text-white" />
		<p class="text-xl font-semibold font-serif mb-2 dark:text-white">
			Drag and drop file(.dbc) here
		</p>
		<p class="text-base font-serif dark:text-white">Or click to select file</p>
	</Dropzone>

	<div class="ms-10 w-1/3 place-self-center grid">
		<div class="self-center">
			<p class="ms-7 text-xl font-semibold font-serif dark:text-white">Uploaded files</p>

			<ol class="ml-10 mt-5 mr-5 h-fit self-center">
				<!-- {#each files.accepted as item}
                    <li>
                        <span class="flex items-center"
                            ><FileOutline class="me-1.5" />{item.name}<AngleRightOutline class="me-1.5" /></span
                        >
                    </li>
                {/each} -->
				<li class="space-y-4">
					<span
						class="p-1 w-full flex items-center font-serif hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
						><FileOutline class="w-10 h-10 me-1.5" />CANDBC_FILE.dbc<AngleRightOutline
							class="me-1.5 ms-auto"
						/></span
					>
					<span
						class="p-1 w-full flex items-center font-serif hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
						><FileOutline class="w-10 h-10 me-1.5" />CANDBC_FILE.dbc<AngleRightOutline
							class="me-1.5  ms-auto"
						/></span
					>
					<span
						class="p-1 w-full flex items-center font-serif hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
						><FileOutline class="w-10 h-10 me-1.5" />CANDBC_FILE.dbc<AngleRightOutline
							class="me-1.5  ms-auto"
						/></span
					>
					<span
						class="p-1 w-full flex items-center font-serif hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
						><FileOutline class="w-10 h-10 me-1.5" />CANDBC_FILE.dbc<AngleRightOutline
							class="me-1.5  ms-auto"
						/></span
					>
					<span
						class="p-1 w-full flex items-center font-serif hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
						><FileOutline class="w-10 h-10 me-1.5" />CANDBC_FILE.dbc<AngleRightOutline
							class="me-1.5  ms-auto"
						/></span
					>
				</li>
			</ol>
		</div>
	</div>
</div>
