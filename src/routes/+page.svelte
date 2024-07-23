<script lang="ts">
	import { g_paths } from '../lib/stores.js';

	import { Button } from 'flowbite-svelte';
	import Dropzone from 'svelte-file-dropzone';
	import { UploadOutline, FileOutline, AngleRightOutline } from 'flowbite-svelte-icons';

	import FileDrop from 'svelte-tauri-filedrop';

	import { invoke } from '@tauri-apps/api/tauri';
	import { trace, info, error, attachConsole } from 'tauri-plugin-log-api';

	let loaded_paths;

	const sub_paths = g_paths.subscribe((paths) => {
		loaded_paths = paths;
	});

	async function open(paths: string[]) {
		info('file open');
		for (const path of paths) {
			info(path);
			await invoke('file_load', { path })
				.then((name) => {
					info('file loaded sucess');
					g_paths.update((prev) => [...prev, name]);
				})
				.catch((err) => {
					error('file loaded failed');
				});
		}
	}

	// update file list when reload
</script>

<div class="mx-48 flex flex-1">
	<FileDrop extensions={['dbc']} handleFiles={open} let:files>
		<div
			class="text-gray-400 bg-gray-100 h-3/4 w-full border-dashed border-indigo-50 border-2 rounded-2xl place-self-center place-content-center place-items-center dark:bg-gray-800 grid"
			class:droppable={files.length > 0}
		>
			<UploadOutline class="w-10 h-10 mb-3 dark:text-white" />
			<p class="text-xl font-semibold font-serif mb-2 dark:text-white">
				Drag and drop file(.dbc) here
			</p>
			<p class="text-base font-serif dark:text-white">Or click to select file</p>
		</div>
	</FileDrop>

	{#if loaded_paths.length > 0}
		<div class="self-center">
			<p class="ms-7 text-xl font-semibold font-serif dark:text-white">Uploaded files</p>

			<ol class="ml-10 mt-5 mr-5 h-fit self-center space-y-2">
				{#each loaded_paths as path}
					<li>
						<Button
							href="/information"
							class="p-1 w-full flex items-center font-serif bg-gray-transparent text-black hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
							><FileOutline class="w-10 h-10 me-1.5" />{path}<AngleRightOutline
								class="me-1.5 ms-auto"
							/></Button
						>
					</li>
				{/each}
			</ol>
		</div>
	{/if}

	<!-- <div class="ms-10 w-1/3 place-self-center grid">
		<div class="self-center">
			<p class="ms-7 text-xl font-semibold font-serif dark:text-white">Uploaded files</p>

			<ol class="ml-10 mt-5 mr-5 h-fit self-center">
				{#each files.accepted as item}
                    <li>
                        <span class="flex items-center"
                            ><FileOutline class="me-1.5" />{item.name}<AngleRightOutline class="me-1.5" /></span
                        >
                    </li>
                {/each}
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
	</div> -->
</div>
