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

<Navbar class="border-b-2">
	<NavBrand href="/">
		<FloppyDiskOutline class="me-2 w-7 h-7" />
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">CANBUS</span>
	</NavBrand>
	<NavUl class="order-1">
		<NavLi href="/information">Information</NavLi>
		<NavLi href="/message">Messages</NavLi>
	</NavUl>
</Navbar>

<slot></slot>
