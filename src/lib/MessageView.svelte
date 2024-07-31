<script lang="ts">
	import * as Carousel from '$lib/components/ui/carousel/index.js';
	import MessageCard from '$lib/MessageCard.svelte';
</script>

<div>
	<div class="dark:text-white">
		<div class="space-y-2">
			<h1 class="font-semibold text-xl">Messages</h1>
			<p class="mt-1 text-base">
				A packet of data on the CAN bus, containing signals and identified by a unique ID.
			</p>
		</div>
		<div class="mt-7 mx-12">
			<Carousel.Root class="w-full">
				<Carousel.Content class="-ml-1">
					<Carousel.Item class="pl-1 md:basis-1/2 lg:basis-1/3">
						<MessageCard />
					</Carousel.Item>
					<!-- <Carousel.Item class="pl-1 md:basis-1/2 lg:basis-1/3">
						<MessageCard />
					</Carousel.Item> -->
				</Carousel.Content>
				<Carousel.Previous />
				<Carousel.Next />
			</Carousel.Root>
		</div>
	</div>
	<div class="mt-20 content-center place-self-end">
		<div class="space-y-2">
			<h1 class="font-semibold text-xl dark:text-white">Signals</h1>
			<p class="mt-1 dark:text-white">
				A specific data point within a message, representing a measurable value like speed or
				temperature.
			</p>
		</div>
	</div>
</div>

<!-- <script lang="ts">
	import { page } from '$app/stores';
	import Carousel from 'svelte-carousel';
	import { Card, Button } from 'flowbite-svelte';
	import {
		ChevronLeftOutline,
		ChevronRightOutline,
		QuestionCircleOutline
	} from 'flowbite-svelte-icons';
	import { Pagination, PaginationItem } from 'flowbite-svelte';

	let pages = [
		{ name: 1, href: '/components/pagination?page=6' },
		{ name: 2, href: '/components/pagination?page=7' },
		{ name: 3, href: '/components/pagination?page=8' },
		{ name: 4, href: '/components/pagination?page=9' },
		{ name: 5, href: '/components/pagination?page=10' }
	];
	$: activeUrl = $page.url.searchParams.get('page');
	$: activeUrl = $page.url.pathname;

	$: {
		pages.forEach((page) => {
			let splitUrl = page.href.split('?');
			let queryString = splitUrl.slice(1).join('?');
			const hrefParams = new URLSearchParams(queryString);
			let hrefValue = hrefParams.get('page');
			if (hrefValue === activeUrl) {
				page.active = true;
			} else {
				page.active = false;
			}
		});
		pages = pages;
	}

	const previous = () => {
		alert('Previous btn clicked. Make a call to your server to fetch data.');
	};
	const next = () => {
		alert('Next btn clicked. Make a call to your server to fetch data.');
	};
</script>

<div>
	<div class="dark:text-white">
		<div class="space-y-2">
			<h1 class="font-semibold text-xl">Messages</h1>
			<p class="mt-1 text-base">
				A packet of data on the CAN bus, containing signals and identified by a unique ID.
			</p>
		</div>
		<div class="m-2">
			<Carousel particlesToShow={5} particlesToScroll={1}>
				<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch dark:text-white">
					<h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						DRIVER_HEARTBEAT
					</h5>
					<div class="space-y-1">
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							ID: 100
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							DLC: 10
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							Sender: K16_BECM | K16_BECM
						</p>
					</div>
					<Button
						color="dark"
						class="w-64 text-white place-self-center self-end text-base dark:bg-gray-600"
					>
						View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>
				<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
					<h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						WebData_1840
					</h5>
					<div class="space-y-1">
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							ID: 200
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							DLC: 10
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							Sender: K16_BECM | K16_BECM
						</p>
					</div>
					<Button
						color="dark"
						class="w-64 text-white place-self-center self-end text-base dark:bg-gray-600"
					>
						View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>

				<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
					<h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						Battery_Module_1
					</h5>
					<div class="space-y-1">
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							ID: 1000
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							DLC: 10
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							Sender: K16_BECM | K16_BECM
						</p>
					</div>
					<Button
						color="dark"
						class="w-64 text-white place-self-center self-end text-base dark:bg-gray-600"
					>
						View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>

				<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
					<h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						Battery_Module_2
					</h5>
					<div class="space-y-1">
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							ID: 500
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							DLC: 10
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							Sender: K16_BECM | K16_BECM
						</p>
					</div>
					<Button
						color="dark"
						class="w-64 text-white place-self-center self-end text-base dark:bg-gray-600"
					>
						View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>

				<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
					<h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						Motor_Module_1
					</h5>
					<div class="space-y-1">
						<p class="font-normal text-gray-700 dark:text-white leading-tight">ID: 50</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							DLC: 10
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							Sender: K16_BECM | K16_BECM
						</p>
					</div>
					<Button
						color="dark"
						class="w-64 text-white place-self-center self-end text-base dark:bg-gray-600"
					>
						View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>

				<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
					<h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						Motor_Module_2 | 60
					</h5>
					<div class="space-y-1">
						<p class="font-normal text-gray-700 dark:text-white leading-tight">ID: 60</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							DLC: 10
						</p>
						<p class="font-normal text-gray-700 dark:text-white leading-tight">
							Sender: K16_BECM | K16_BECM
						</p>
					</div>
					<Button
						color="dark"
						class="w-64 text-white place-self-center self-end text-base dark:bg-gray-600"
					>
						View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>
			</Carousel>
		</div>
	</div>
	<div class="mt-20 content-center place-self-end">
		<div class="space-y-2">
			<h1 class="font-semibold text-xl dark:text-white">Signals</h1>
			<p class="mt-1 dark:text-white">
				A specific data point within a message, representing a measurable value like speed or
				temperature.
			</p>
		</div>

		<div class="table-wrp block m-2">
			<table class="mt-5 w-full">
				<thead
					class="bg-gray-50 border-b sticky top-0 text-sm font-semibold dark:bg-gray-700 dark:text-gray-400 text-gray-700 text-left"
				>
					<tr class="w-full">
						<th scope="col" class="p-4">
							<div class="flex items-center">
								<input
									id="checkbox-all-search"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-all-search" class="sr-only">checkbox</label>
							</div>
						</th>
						<th scope="col" class="px-6 py-4"> Name </th>
						<th scope="col" class="px-6 py-4"> Multiplexer </th>
						<th scope="col" class="px-6 py-4"> Bit (Start | Size) </th>
						<th scope="col" class="px-6 py-4"> Byte Order </th>
						<th scope="col" class="px-6 py-4"> Value Type </th>
						<th scope="col" class="px-6 py-4"> Scale | Offset </th>
						<th scope="col" class="px-6 py-4"> Min | Max </th>
						<th scope="col" class="px-6 py-4"> Unit </th>
						<th scope="col" class="px-6 py-4"> Receiver </th>
					</tr>
				</thead>
				<tbody class="overflow-y-auto">
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
					<tr
						class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100 dark:bg-gray-800"
					>
						<td class="w-4 p-4">
							<div class="flex items-center">
								<input
									id="checkbox-table-search-1"
									type="checkbox"
									class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
								/>
								<label for="checkbox-table-search-1" class="sr-only">checkbox</label>
							</div>
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white">
							Name
						</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Multiplexer</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.0 | 0.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>LittleEndian</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Unsigned</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>0.123 | 123.1</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>-100.0 | 200.0</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>km/h</td
						>
						<td class="text-sm text-gray-900 font-light px-6 py-4 dark:text-white"
							>Recv1, Recv2</td
						>
					</tr>
				</tbody>
			</table>

			<div class="mt-7 grid justify-items-center">
				<Pagination {pages} on:previous={previous} on:next={next} icon>
					<svelte:fragment slot="prev">
						<span class="sr-only">Previous</span>
						<ChevronLeftOutline class="w-2.5 h-2.5" />
					</svelte:fragment>
					<svelte:fragment slot="next">
						<span class="sr-only">Next</span>
						<ChevronRightOutline class="w-2.5 h-2.5" />
					</svelte:fragment>
				</Pagination>
			</div>
		</div>
	</div>
</div> -->
