<script lang="ts">
	import { g_dbcs, g_selected_dbc } from '$lib/stores.js';
	import { select_dbc } from '../../lib/common.js';

	import { invoke } from '@tauri-apps/api/tauri';

	import { info, error } from 'tauri-plugin-log-api';

	import { Upload, File, ChevronRight, Folder } from 'lucide-svelte';

	import * as Resizable from '$lib/components/ui/resizable';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Separator } from '$lib/components/ui/separator';

	let dbcs;
	const sub_dbcs = g_dbcs.subscribe((dbc) => {
		dbcs = dbc;
	});
</script>

<Resizable.PaneGroup direction="horizontal" class="flex-1 flex">
	<Resizable.Pane defaultSize={15} class="flex flex-col m-3">
		<div class="flex flex-col place-content-between h-full">
			<div class="w-full">
				<div class="flex">
					<Folder
						class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
					/>
					<p
						class="ml-2 w-full overflow-hidden text-ellipsis whitespace-nowrap text-lg font-semibold"
					>
						Uploaded DBC
					</p>
				</div>
			</div>

			<div class="w-full h-full mt-1 overflow-y-auto">
				<ol>
					{#each dbcs as dbc}
						<li class="w-full">
							<Button
								on:click={() => select_dbc(dbc)}
								class="flex w-full h-fit text-left bg-transparent text-black hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
							>
								<File
									class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
								/>
								<p class="ml-1 w-full overflow-hidden text-ellipsis whitespace-nowrap text-base">
									{dbc}
								</p>
							</Button>
						</li>
					{/each}
				</ol>
			</div>

			<Separator />
			<Button
				class="mt-3 flex text-left text-black bg-transparent hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
			>
				<Upload
					class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
				/>
				<p
					class="ml-2 w-full overflow-hidden text-ellipsis whitespace-nowrap text-lg font-semibold"
				>
					ADD DBC
				</p>
			</Button>
		</div>
	</Resizable.Pane>
	<Resizable.Handle withHandle />
	<Resizable.Pane defaultSize={85} class="flex flex-col m-3">
		<slot />
	</Resizable.Pane>
</Resizable.PaneGroup>
