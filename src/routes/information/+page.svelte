<script>
	import { page } from '$app/stores';
	import { Pagination, PaginationItem } from 'flowbite-svelte';
	import { Tabs, TabItem } from 'flowbite-svelte';
	import { Card, Button } from 'flowbite-svelte';
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

	import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
	import {
		ChartPieSolid,
		GridSolid,
		MailBoxSolid,
		UserSolid,
		ArrowRightToBracketOutline,
		EditOutline
	} from 'flowbite-svelte-icons';
	let spanClass = 'flex-1 ms-3 whitespace-nowrap';
	$: activeUrl = $page.url.pathname;
</script>

<div class="m-10">
	<Tabs>
		<TabItem open title="Information">
			<div>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					<b>BDC file name</b>
				</p>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					<b>version</b>
				</p>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					<b>Symbols</b>
				</p>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					<b>nodes</b>
				</p>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					<b>summary: messages cnt (signal cnt)</b>
				</p>
				<p class="text-sm text-gray-500 dark:text-gray-400">
					<b>comments</b>
				</p>
			</div>
		</TabItem>
		<TabItem title="Messages">
			<div class="mx-3">
				<div class="space-y-2">
					<h1 class="font-semibold text-4xl font-serif">Messages</h1>
					<p class="mt-1 font-serif">
						A packet of data on the CAN bus, containing signals and identified by a unique ID.
					</p>
				</div>
				<div class="m-2">
					<Carousel particlesToShow={4} particlesToScroll={1}>
						<Card class="h-56 m-5 bg-gray-200 grid justify-items-stretch">
							<h5
								class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
							>
								DRIVER_HEARTBEAT
							</h5>
							<div class="space-y-1">
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									ID: 100
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									DLC: 10
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									Sender: K16_BECM | K16_BECM
								</p>
							</div>
							<Button
								color="dark"
								class="w-64 font-serif text-white place-self-center self-end text-base"
							>
								View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
							</Button>
						</Card>

						<Card class="h-56 m-5 bg-gray-200 grid justify-items-stretch">
							<h5
								class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
							>
								WebData_1840
							</h5>
							<div class="space-y-1">
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									ID: 200
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									DLC: 10
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									Sender: K16_BECM | K16_BECM
								</p>
							</div>
							<Button
								color="dark"
								class="w-64 font-serif text-white place-self-center self-end text-base"
							>
								View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
							</Button>
						</Card>

						<Card class="h-56 m-5 bg-gray-200 grid justify-items-stretch">
							<h5
								class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
							>
								Battery_Module_1
							</h5>
							<div class="space-y-1">
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									ID: 1000
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									DLC: 10
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									Sender: K16_BECM | K16_BECM
								</p>
							</div>
							<Button
								color="dark"
								class="w-64 font-serif text-white place-self-center self-end text-base"
							>
								View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
							</Button>
						</Card>

						<Card class="h-56 m-5 bg-gray-200 grid justify-items-stretch">
							<h5
								class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
							>
								Battery_Module_2
							</h5>
							<div class="space-y-1">
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									ID: 500
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									DLC: 10
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									Sender: K16_BECM | K16_BECM
								</p>
							</div>
							<Button
								color="dark"
								class="w-64 font-serif text-white place-self-center self-end text-base"
							>
								View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
							</Button>
						</Card>

						<Card class="h-56 m-5 bg-gray-200 grid justify-items-stretch">
							<h5
								class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
							>
								Motor_Module_1
							</h5>
							<div class="space-y-1">
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									ID: 50
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									DLC: 10
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									Sender: K16_BECM | K16_BECM
								</p>
							</div>
							<Button
								color="dark"
								class="w-64 font-serif text-white place-self-center self-end text-base"
							>
								View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
							</Button>
						</Card>

						<Card class="h-56 m-5 bg-gray-200 grid justify-items-stretch">
							<h5
								class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white font-serif"
							>
								Motor_Module_2 | 60
							</h5>
							<div class="space-y-1">
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									ID: 60
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									DLC: 10
								</p>
								<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight font-serif">
									Sender: K16_BECM | K16_BECM
								</p>
							</div>
							<Button
								color="dark"
								class="w-64 font-serif text-white place-self-center self-end text-base"
							>
								View Signals <QuestionCircleOutline class="w-6 h-6 ms-2 text-white" />
							</Button>
						</Card>
					</Carousel>
				</div>
			</div>
			<div class="mt-11 content-center">
				<div class="space-y-2">
					<h1 class="font-semibold text-4xl font-serif">Signals</h1>
					<p class="mt-1 font-serif">
						A specific data point within a message, representing a measurable value like speed or
						temperature.
					</p>
				</div>

				<div class="table-wrp block m-2">
					<table class="w-full">
						<thead class="bg-white border-b sticky top-0">
							<tr class="w-full">
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Name
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Multiplexer
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Bit (Start | Size)
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Byte Order
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Value Type
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Scale | Offset
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Min | Max
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Unit
								</th>
								<th
									scope="col"
									class="text-sm font-semibold text-gray-900 px-6 py-4 text-left font-serif"
								>
									Receiver
								</th>
							</tr>
						</thead>
						<tbody class="overflow-y-auto">
							<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif"> Name </td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Multiplexer</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.0 | 0.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">LittleEndian</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Unsigned</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.123 | 123.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">-100.0 | 200.0</td
								>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">km/h</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Recv1, Recv2</td>
							</tr>
							<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif"> Name </td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Multiplexer</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.0 | 0.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">LittleEndian</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Unsigned</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.123 | 123.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">-100.0 | 200.0</td
								>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">km/h</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Recv1, Recv2</td>
							</tr>
							<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif"> Name </td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Multiplexer</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.0 | 0.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">LittleEndian</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Unsigned</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.123 | 123.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">-100.0 | 200.0</td
								>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">km/h</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Recv1, Recv2</td>
							</tr>
							<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif"> Name </td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Multiplexer</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.0 | 0.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">LittleEndian</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Unsigned</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.123 | 123.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">-100.0 | 200.0</td
								>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">km/h</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Recv1, Recv2</td>
							</tr>
							<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif"> Name </td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Multiplexer</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.0 | 0.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">LittleEndian</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Unsigned</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">0.123 | 123.1</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">-100.0 | 200.0</td
								>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">km/h</td>
								<td class="text-sm text-gray-900 font-light px-6 py-4 font-serif">Recv1, Recv2</td>
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
		</TabItem>
	</Tabs>
</div>
