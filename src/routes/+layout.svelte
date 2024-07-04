<script>
	import '../app.css';
	import {
		Navbar,
		NavBrand,
		NavLi,
		NavUl,
		NavHamburger,
		Button,
		Input,
		Card
	} from 'flowbite-svelte';
	import { Tabs, TabItem } from 'flowbite-svelte';
	import {
		UserCircleSolid,
		GridSolid,
		AdjustmentsVerticalSolid,
		ClipboardSolid,
		FloppyDiskOutline
	} from 'flowbite-svelte-icons';
	import { ArrowRightOutline } from 'flowbite-svelte-icons';
</script>

<Navbar class="border-b-2 font-serif h-1/10">
	<NavBrand href="/">
		<FloppyDiskOutline class="me-2 w-7 h-7" />
		<span class="self-center whitespace-nowrap text-2xl font-semibold dark:text-white">CANBUS</span>
	</NavBrand>
	<NavUl class="order-1">
		<NavLi href="/" class="text-lg">Load</NavLi>
		<NavLi href="/information" class="text-lg">Information</NavLi>
		<NavLi href="/message" class="text-lg">Messages</NavLi>
	</NavUl>
</Navbar>

<slot></slot>
