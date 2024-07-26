<script lang="ts">
	import { page } from '$app/stores';
	import { Pagination, PaginationItem } from 'flowbite-svelte';
	import { Tabs, TabItem } from 'flowbite-svelte';
	import { Card, Button } from 'flowbite-svelte';
	import { Badge } from 'flowbite-svelte';
	import { Listgroup } from 'flowbite-svelte';
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import Carousel from 'svelte-carousel';
	import {
		ArrowRightOutline,
		ChevronLeftOutline,
		ChevronRightOutline,
		SearchOutline,
		QuestionCircleOutline
	} from 'flowbite-svelte-icons';

	$: activeUrl = $page.url.searchParams.get('page');
	let pages = [
		{ name: 1, href: '/components/pagination?page=6' },
		{ name: 2, href: '/components/pagination?page=7' },
		{ name: 3, href: '/components/pagination?page=8' },
		{ name: 4, href: '/components/pagination?page=9' },
		{ name: 5, href: '/components/pagination?page=10' }
	];

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

	let spanClass = 'flex-1 ms-3 whitespace-nowrap';
	$: activeUrl = $page.url.pathname;
</script>

<div class="m-5">
	<Tabs contentClass="m-10">
		<TabItem open title="Information">
			<div class="space-y-5 text-xl font-semibold text-gray-500 dark:text-white">
				<div>
					<p>DBC file:</p>
					<div class="mt-1">
						<Badge color="dark" class="text-lg">CANDBC_FILE_.dbc</Badge>
					</div>
				</div>
				<div>
					<p>Version:</p>
					<div class="mt-1">
						<Badge color="dark" class="mt-1 text-lg">0.1.0</Badge>
					</div>
				</div>
				<div>
					<p>Symbols:</p>
					<div class="mt-1 space-x-1">
						<Badge color="dark" class="text-lg">CM_</Badge>
						<Badge color="dark" class="text-lg">BA_</Badge>
						<Badge color="dark" class="text-lg">VAL_</Badge>
						<Badge color="dark" class="text-lg">CAT_</Badge>
					</div>
				</div>
				<div>
					<p>Nodes:</p>
					<div class="mt-1 space-x-1">
						<Badge color="dark" class="text-lg">K16_BECM</Badge>
						<Badge color="dark" class="text-lg">K114B_HPCM</Badge>
						<Badge color="dark" class="text-lg">T18_BatteryCharger</Badge>
					</div>
				</div>
				<div>
					<p>Messages:</p>
					<div class="mt-1 space-x-1">
						<Badge color="dark" class="text-lg">WebData_1840 (3)</Badge>
						<Badge color="dark" class="text-lg">Battery_Module_1 (10)</Badge>
						<Badge color="dark" class="text-lg">Battery_Module_2 (15)</Badge>
					</div>
				</div>
				<div>
					<p>Comments:</p>
					<Listgroup class="w-fit bg-transparent dark:bg-transparent border-0 divide-y-2">
						<div class="py-4 space-x-1">
							<Badge color="green" class="text-lg">BO_</Badge>
							<Badge color="blue" class="text-lg">1840</Badge>
							<Badge color="dark" class="text-lg">Some Message comment</Badge>
						</div>
						<div class="py-4 space-x-1">
							<Badge color="green" class="text-lg">SG_</Badge>
							<Badge color="blue" class="text-lg">1840</Badge>
							<Badge color="blue" class="text-lg">Signal_4</Badge>
							<Badge color="dark" class="text-lg"
								>asaklfjlsdfjlsdfglsHH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv</Badge
							>
						</div>
						<div class="py-4 space-x-1">
							<Badge color="green" class="text-lg">SG_</Badge>
							<Badge color="blue" class="text-lg">5</Badge>
							<Badge color="dark" class="text-lg">TestSigLittleUnsigned1</Badge>
							<Badge color="dark" class="text-lg"
								>asaklfjlsdfjlsdfgls=0943503450KFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv</Badge
							>
						</div>
						<div class="py-4 space-x-1">
							<Badge color="dark" class="text-lg">Imported file _honda_common.dbc starts here</Badge
							>
						</div>
						<div class="py-4 space-x-1">
							<Badge color="green" class="text-lg">BU_</Badge>
							<Badge color="blue" class="text-lg">K17_EBCM</Badge>
							<Badge color="dark" class="text-lg">Electronic Brake Control Module</Badge>
						</div>
					</Listgroup>
				</div>
			</div>
		</TabItem>
		<TabItem title="Messages">
			<div>
				<div class="dark:text-white">
					<div class="space-y-2">
						<h1 class="font-semibold text-4xl font-serif">Messages</h1>
						<p class="mt-1 font-serif">
							A packet of data on the CAN bus, containing signals and identified by a unique ID.
						</p>
					</div>
					<div class="m-2">
						<Carousel particlesToShow={5} particlesToScroll={1}>
							<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch dark:text-white">
								<h5
									class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
								>
									DRIVER_HEARTBEAT
								</h5>
								<div class="space-y-1">
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										ID: 100
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										DLC: 10
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										Sender: K16_BECM | K16_BECM
									</p>
								</div>
								<Button
									color="dark"
									class="w-64 font-serif text-white place-self-center self-end text-base dark:bg-gray-600"
								>
									View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
								</Button>
							</Card>
							<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
								<h5
									class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
								>
									WebData_1840
								</h5>
								<div class="space-y-1">
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										ID: 200
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										DLC: 10
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										Sender: K16_BECM | K16_BECM
									</p>
								</div>
								<Button
									color="dark"
									class="w-64 font-serif text-white place-self-center self-end text-base dark:bg-gray-600"
								>
									View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
								</Button>
							</Card>

							<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
								<h5
									class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
								>
									Battery_Module_1
								</h5>
								<div class="space-y-1">
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										ID: 1000
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										DLC: 10
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										Sender: K16_BECM | K16_BECM
									</p>
								</div>
								<Button
									color="dark"
									class="w-64 font-serif text-white place-self-center self-end text-base dark:bg-gray-600"
								>
									View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
								</Button>
							</Card>

							<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
								<h5
									class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
								>
									Battery_Module_2
								</h5>
								<div class="space-y-1">
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										ID: 500
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										DLC: 10
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										Sender: K16_BECM | K16_BECM
									</p>
								</div>
								<Button
									color="dark"
									class="w-64 font-serif text-white place-self-center self-end text-base dark:bg-gray-600"
								>
									View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
								</Button>
							</Card>

							<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
								<h5
									class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
								>
									Motor_Module_1
								</h5>
								<div class="space-y-1">
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										ID: 50
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										DLC: 10
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										Sender: K16_BECM | K16_BECM
									</p>
								</div>
								<Button
									color="dark"
									class="w-64 font-serif text-white place-self-center self-end text-base dark:bg-gray-600"
								>
									View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
								</Button>
							</Card>

							<Card class="h-64 m-5 bg-gray-200 grid justify-items-stretch">
								<h5
									class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
								>
									Motor_Module_2 | 60
								</h5>
								<div class="space-y-1">
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										ID: 60
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										DLC: 10
									</p>
									<p class="font-normal text-gray-700 dark:text-white leading-tight font-serif">
										Sender: K16_BECM | K16_BECM
									</p>
								</div>
								<Button
									color="dark"
									class="w-64 font-serif text-white place-self-center self-end text-base dark:bg-gray-600"
								>
									View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
								</Button>
							</Card>
						</Carousel>
					</div>
				</div>
				<div class="mt-20 content-center place-self-end">
					<div class="space-y-2">
						<h1 class="font-semibold text-4xl font-serif dark:text-white">Signals</h1>
						<p class="mt-1 font-serif dark:text-white">
							A specific data point within a message, representing a measurable value like speed or
							temperature.
						</p>
					</div>

					<div class="table-wrp block m-2">
						<table class="mt-5 w-full">
							<thead
								class="bg-gray-50 border-b sticky top-0 text-sm font-semibold dark:bg-gray-700 dark:text-gray-400 text-gray-700 font-serif text-left"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white">
										Name
									</td>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Multiplexer</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.0 | 0.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>LittleEndian</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>Unsigned</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>0.123 | 123.1</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>-100.0 | 200.0</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
										>km/h</td
									>
									<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif dark:text-white"
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
			</div>
		</TabItem>
	</Tabs>
</div>
