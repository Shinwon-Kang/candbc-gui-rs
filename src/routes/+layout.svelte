<script>
	import '../app.css';
	import { Navbar, NavBrand, NavLi, NavUl } from 'flowbite-svelte';
	import { FloppyDiskOutline } from 'flowbite-svelte-icons';
	import { Badge } from 'flowbite-svelte';
	import { DarkMode } from 'flowbite-svelte';
</script>

<div class="h-full flex flex-col">
	<Navbar class="border-b-2 font-serif">
		<NavBrand href="/information">
			<FloppyDiskOutline class="me-2 w-7 h-7" />
			<span class="self-center whitespace-nowrap text-2xl font-semibold dark:text-white"
				>CANBUS</span
			>
		</NavBrand>
		<NavUl class="order-1">
			<!-- <NavLi href="/" class="text-lg">Load</NavLi> -->
			<!-- <NavLi href="/information" class="text-lg">Information</NavLi> -->
			<!-- <NavLi href="/message" class="text-lg">Messages</NavLi> -->
			<!-- <Badge href="/information" large color="green" class="h-10 w-fit"
				><span><p class="text-semibold font-serif text-base">CANDBC_FILE.dbc</p></span></Badge
			> -->
			<DarkMode />
		</NavUl>
	</Navbar>

	<div class="flex-1 flex-col flex dark:bg-gray-900">
		<slot />
	</div>
</div>
