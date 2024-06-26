<script>
	import Carousel from 'svelte-carousel';
	import { Card, Button } from 'flowbite-svelte';
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import { ArrowRightOutline } from 'flowbite-svelte-icons';
</script>

<div style="background-color:yellowgreen" class="content-center">
	<div style="background-color:aqua">
		<h1 class="font-semibold">Messages</h1>
	</div>

	<div style="background-color:magenta">
		<Carousel particlesToShow={3} particlesToScroll={1}>
			<Card>
				<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
					CANBUS1
				</h5>
				<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
					Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
					chronological order.
				</p>
				<Button class="w-fit">
					Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
				</Button>
			</Card>

			<Card>
				<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
					CANBUS2
				</h5>
				<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
					Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
					chronological order.
				</p>
				<Button class="w-fit">
					Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
				</Button>
			</Card>

			<Card>
				<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
					CANBUS3
				</h5>
				<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
					Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
					chronological order.
				</p>
				<Button class="w-fit">
					Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
				</Button>
			</Card>

			<Card>
				<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
					CANBUS4
				</h5>
				<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
					Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
					chronological order.
				</p>
				<Button class="w-fit">
					Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
				</Button>
			</Card>

			<Card>
				<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
					CANBUS5
				</h5>
				<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
					Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
					chronological order.
				</p>
				<Button class="w-fit">
					Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
				</Button>
			</Card>
		</Carousel>
	</div>

	<div style="background-color:chartreuse">
		<h1 class="font-semibold">Signals</h1>
	</div>

	<div style="background-color:tomato">
		<Table>
			<TableHead>
				<TableHeadCell>Name</TableHeadCell>
				<TableHeadCell>Multiplexer</TableHeadCell>
				<TableHeadCell>Start Bit</TableHeadCell>
				<TableHeadCell>Bit Size</TableHeadCell>
				<TableHeadCell>Byte Order</TableHeadCell>
				<TableHeadCell>Value Type</TableHeadCell>
				<TableHeadCell>Scale</TableHeadCell>
				<TableHeadCell>Offset</TableHeadCell>
				<TableHeadCell>Min | Max</TableHeadCell>
				<TableHeadCell>Unit</TableHeadCell>
				<TableHeadCell>Receiver</TableHeadCell>
			</TableHead>
			<TableBody tableBodyClass="divide-y">
				<TableBodyRow>
					<TableBodyCell>Apple MacBook Pro 17"</TableBodyCell>
					<TableBodyCell>Sliver</TableBodyCell>
					<TableBodyCell>Laptop</TableBodyCell>
					<TableBodyCell>$2999</TableBodyCell>
					<TableBodyCell>Microsoft Surface Pro</TableBodyCell>
					<TableBodyCell>White</TableBodyCell>
					<TableBodyCell>Laptop PC</TableBodyCell>
					<TableBodyCell>$1999</TableBodyCell>
					<TableBodyCell>Microsoft Surface Pro</TableBodyCell>
					<TableBodyCell>km/s</TableBodyCell>
					<TableBodyCell>Laptop PC</TableBodyCell>
				</TableBodyRow>
				<TableBodyRow>
					<TableBodyCell>Apple MacBook Pro 17"</TableBodyCell>
					<TableBodyCell>Sliver</TableBodyCell>
					<TableBodyCell>Laptop</TableBodyCell>
					<TableBodyCell>$2999</TableBodyCell>
					<TableBodyCell>Microsoft Surface Pro</TableBodyCell>
					<TableBodyCell>White</TableBodyCell>
					<TableBodyCell>Laptop PC</TableBodyCell>
					<TableBodyCell>$1999</TableBodyCell>
					<TableBodyCell>Microsoft Surface Pro</TableBodyCell>
					<TableBodyCell>km/s</TableBodyCell>
					<TableBodyCell>Laptop PC</TableBodyCell>
				</TableBodyRow>
				<TableBodyRow>
					<TableBodyCell>Apple MacBook Pro 17"</TableBodyCell>
					<TableBodyCell>Sliver</TableBodyCell>
					<TableBodyCell>Laptop</TableBodyCell>
					<TableBodyCell>$2999</TableBodyCell>
					<TableBodyCell>Microsoft Surface Pro</TableBodyCell>
					<TableBodyCell>White</TableBodyCell>
					<TableBodyCell>Laptop PC</TableBodyCell>
					<TableBodyCell>$1999</TableBodyCell>
					<TableBodyCell>Microsoft Surface Pro</TableBodyCell>
					<TableBodyCell>km/s</TableBodyCell>
					<TableBodyCell>Laptop PC</TableBodyCell>
				</TableBodyRow>
			</TableBody>
		</Table>
	</div>
</div>
