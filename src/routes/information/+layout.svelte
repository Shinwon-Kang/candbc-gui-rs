<script lang="ts">
	import {
		Sidebar,
		SidebarGroup,
		SidebarItem,
		SidebarWrapper,
		SidebarBrand
	} from 'flowbite-svelte';
	import {
		ChartPieSolid,
		GridSolid,
		MailBoxSolid,
		UserSolid,
		ShoppingBagSolid,
		ArrowRightToBracketOutline,
		EditOutline,
		FireSolid,
		BookSolid,
		RestoreWindowOutline,
		LifeSaverSolid,
		FileOutline,
		UploadOutline
	} from 'flowbite-svelte-icons';
	let spanClass = 'flex-1 ms-3 whitespace-nowrap';
</script>

<div class="mt-5 flex-1 flex">
	<div class="m-5 border-r-2 border-gray-200 dark:border-gray-600">
		<Sidebar asideClass="h-full w-64 flex flex-col items-center dark:text-white">
			<p class="w-full mt-3 text-2xl font-semibold">Uploaded DBC</p>
			<SidebarWrapper divClass="w-full flex-1 grid content-between">
				<SidebarGroup ulClass="mt-5 mr-7 space-y-4 self-start">
					<SidebarItem label="CANDBC_FILE.dbc" href="/">
						<svelte:fragment slot="icon">
							<FileOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
					<SidebarItem label="CANDBC_FILE.dbc">
						<svelte:fragment slot="icon">
							<FileOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
					<SidebarItem label="CANDBC_FILE.dbc">
						<svelte:fragment slot="icon">
							<FileOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
					<SidebarItem label="CANDBC_FILE.dbc">
						<svelte:fragment slot="icon">
							<FileOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
					<SidebarItem label="CANDBC_FILE.dbc">
						<svelte:fragment slot="icon">
							<FileOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
					<SidebarItem label="CANDBC_FILE.dbc">
						<svelte:fragment slot="icon">
							<FileOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
					<SidebarItem label="CANDBC_FILE.dbc">
						<svelte:fragment slot="icon">
							<FileOutline
								class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							/>
						</svelte:fragment>
					</SidebarItem>
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
</div>
