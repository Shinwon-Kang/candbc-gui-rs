<script lang="ts">
	import { g_paths } from '$lib/stores.js';
	import { invoke } from '@tauri-apps/api/tauri';

	import { info, error } from 'tauri-plugin-log-api';

	import { Upload, File } from 'lucide-svelte';

	import * as Resizable from '$lib/components/ui/resizable';
	import { Button } from '$lib/components/ui/button/index.js';

	let loaded_paths;
	const sub_paths = g_paths.subscribe((paths) => {
		loaded_paths = paths;
	});

	async function select(path) {
		info(path);
		await invoke('return_json')
			.then((json) => {
				info('good');
			})
			.catch((err) => {
				error('error return json');
			});
	}
</script>

<Resizable.PaneGroup direction="horizontal" class="flex-1 flex">
	<Resizable.Pane defaultSize={15} class="flex flex-col">
		<div class="h-full mt-7 m-3 grid grid-rows-1 grid-cols-1 place-content-between">
			<div class="space-y-3 w-full">
				<p class="text-xl font-semibold overflow-hidden text-ellipsis whitespace-nowrap">
					Uploaded DBC
				</p>
				<ol class="space-y-1">
					{#each loaded_paths as path}
						<li>
							<Button
								on:click={() => select(path)}
								class="flex w-full text-left bg-gray-transparent text-black hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
							>
								<File
									class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
								/>
								<p class="ml-1 w-full overflow-hidden text-ellipsis whitespace-nowrap text-base">
									{path}
								</p>
							</Button>
						</li>
					{/each}
				</ol>
			</div>
			<div class="flex ml-2 mb-2">
				<Upload
					class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
				/>
				<p class="ml-2 w-full overflow-hidden text-ellipsis whitespace-nowrap text-base">ADD DBC</p>
			</div>
		</div>
	</Resizable.Pane>
	<Resizable.Handle withHandle />
	<Resizable.Pane defaultSize={85}>
		<div class="h-full">
			<slot />
		</div>
	</Resizable.Pane>
</Resizable.PaneGroup>
