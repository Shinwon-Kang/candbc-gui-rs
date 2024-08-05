<script lang="ts">
	import { g_dbcs, g_selected_dbc, g_summary_info } from '../lib/stores.js';
	import { select_dbc } from '../lib/common.js'

	import { invoke } from '@tauri-apps/api/tauri';
	import { trace, info, error, attachConsole } from 'tauri-plugin-log-api';

	import { Button } from '$lib/components/ui/button/index.js';
	import { Upload, File, ChevronRight } from 'lucide-svelte';
	import FileDrop from 'svelte-tauri-filedrop';

	let dbcs;
	const sub_paths = g_dbcs.subscribe((paths) => {
		dbcs = paths;
	});

	async function open(paths: string[]) {
		info('file open');
		for (const path of paths) {
			info(path);
			await invoke('file_load', { path })
				.then((name) => {
					info('file loaded sucess');
					g_dbcs.update((prev) => [...prev, name]);
				})
				.catch((err) => {
					error('file loaded failed');
				});
		}
	}
</script>

<div class="mx-48 flex flex-1">
	<FileDrop extensions={['dbc']} handleFiles={open} let:files>
		<div
			class="text-gray-400 bg-gray-100 h-3/4 w-full border-dashed border-indigo-50 border-2 rounded-2xl place-self-center place-content-center place-items-center dark:bg-gray-800 grid"
			class:droppable={files.length > 0}
		>
			<Upload class="w-8 h-8 mb-3 dark:text-white" />
			<p class="text-xl font-semibold font-serif mb-2 dark:text-white">
				Drag and drop file(.dbc) here
			</p>
			<p class="text-base font-serif dark:text-white">Or click to select file</p>
		</div>
	</FileDrop>

	{#if dbcs.length > 0}
		<div class="self-center w-1/2">
			<p class="ms-7 text-xl font-semibold font-serif dark:text-white">Uploaded files</p>

			<ol class="ml-10 mt-5 mr-5 h-fit self-center space-y-2">
				{#each dbcs as dbc}
					<li>
						<Button
							on:click={() => select_dbc(dbc)}
							href="/information"
							class="p-1 flex items-center font-serif bg-gray-transparent text-black hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
							><File class="w-9 h-9 me-1.5" />
							<p class="overflow-hidden text-ellipsis whitespace-nowrap">{dbc}</p>
							<ChevronRight class="me-1.5 ms-auto" /></Button
						>
					</li>
				{/each}
			</ol>
		</div>
	{/if}
</div>
