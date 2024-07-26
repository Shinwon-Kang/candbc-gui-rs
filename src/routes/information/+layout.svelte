<script lang="ts">
	import { g_paths } from '$lib/stores.js';

	import { Upload, File } from 'lucide-svelte';

	import * as Resizable from '$lib/components/ui/resizable';

	let loaded_paths;
	const sub_paths = g_paths.subscribe((paths) => {
		loaded_paths = paths;
	});
</script>

<Resizable.PaneGroup direction="horizontal" class="flex-1 flex">
	<Resizable.Pane defaultSize={15} class="flex flex-col">
		<div class="h-full m-3 grid grid-rows-1 grid-cols-1 place-content-between">
			<div class="space-y-5 w-full">
				<p class="text-xl font-semibold overflow-hidden text-ellipsis whitespace-nowrap">
					Uploaded DBC
				</p>
				<ol class="space-y-3 ml-2">
					{#each loaded_paths as path}
						<li>
							<div class="flex w-full">
								<File
									class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
								/>
								<p class="ml-1 w-full overflow-hidden text-ellipsis whitespace-nowrap text-base">
									{path}
								</p>
							</div>
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
		<div class="">
			<slot />
		</div>
	</Resizable.Pane>
</Resizable.PaneGroup>
