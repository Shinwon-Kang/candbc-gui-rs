<script>
	import '../app.css';
	import { Navbar, NavBrand, NavLi, NavUl } from 'flowbite-svelte';
	import { FloppyDiskOutline } from 'flowbite-svelte-icons';
	import { Badge } from 'flowbite-svelte';
</script>

<Navbar class="border-b-2 font-serif h-1/10">
	<NavBrand href="/">
		<FloppyDiskOutline class="me-2 w-7 h-7" />
		<span class="self-center whitespace-nowrap text-2xl font-semibold dark:text-white">CANBUS</span>
	</NavBrand>
	<NavUl class="order-1">
		<!-- <NavLi href="/" class="text-lg">Load</NavLi> -->
		<!-- <NavLi href="/information" class="text-lg">Information</NavLi> -->
		<!-- <NavLi href="/message" class="text-lg">Messages</NavLi> -->
		<Badge href="/information" large color="green" class="h-10 w-fit"
			><span><p class="text-semibold font-serif text-base">CANDBC_FILE.dbc</p></span></Badge
		>
	</NavUl>
</Navbar>

<slot></slot>
