<script lang="ts">
	import { g_paths } from '$lib/stores.js';

	import {
		Sidebar,
		SidebarGroup,
		SidebarItem,
		SidebarWrapper,
		SidebarBrand
	} from 'flowbite-svelte';
	import { FileOutline, UploadOutline } from 'flowbite-svelte-icons';

	import * as Resizable from "$lib/components/ui/resizable";

	let loaded_paths;
	const sub_paths = g_paths.subscribe((paths) => {
		loaded_paths = paths;
	});
</script>

<Resizable.PaneGroup
  direction="horizontal"
  class="min-h-[200px] max-w-md rounded-lg border"
>
  <Resizable.Pane defaultSize={25}>
    <div class="flex h-full items-center justify-center p-6">
      <span class="font-semibold">Sidebar</span>
    </div>
  </Resizable.Pane>
  <Resizable.Handle withHandle />
  <Resizable.Pane defaultSize={75}>
    <div class="flex h-full items-center justify-center p-6">
      <span class="font-semibold">Content</span>
    </div>
  </Resizable.Pane>
</Resizable.PaneGroup>

<!-- <div class="mt-5 flex-1 flex">
	<div class="m-5 border-r-2 border-gray-200 dark:border-gray-600 resize-x">
		<Sidebar asideClass="h-full w-64 flex flex-col items-center dark:text-white">
			<p class="w-full mt-3 text-2xl font-semibold">Uploaded DBC</p>
			<SidebarWrapper divClass="w-full flex-1 grid content-between">
				<SidebarGroup ulClass="mt-5 mr-7 space-y-4 self-start">
					{#each loaded_paths as path}
						<SidebarItem href="/">
							<svelte:fragment slot="subtext">
								<p class="break-all">{path}</p>
							</svelte:fragment>
							<svelte:fragment slot="icon">
								<FileOutline
									class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
								/>
							</svelte:fragment>
						</SidebarItem>
					{/each}
				</SidebarGroup>
				<SidebarGroup border ulClass="mr-7">
					<SidebarItem label="ADD DBC">
						<svelte:fragment slot="icon">
							<UploadOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
				</SidebarGroup>
			</SidebarWrapper>
		</Sidebar>
	</div>

	<div class="grow w-0 flex-1 flex flex-col">
		<slot />
	</div>
</div> -->
