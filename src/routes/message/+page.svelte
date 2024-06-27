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

<div
	style="background-color:black"
	class="content-center min-h-full mx-auto my-auto w-4/5 place-content-center"
>
	<div>
		<div style="background-color:aqua">
			<h1 class="font-semibold text-2xl font-serif">Messages</h1>
		</div>
		<div style="background-color:magenta">
			<Carousel particlesToShow={4} particlesToScroll={1}>
				<Card class="m-5">
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

				<Card class="m-5">
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

				<Card class="m-5">
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

				<Card class="m-5">
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

				<Card class="m-5">
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

				<Card class="m-5">
					<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						CANBUS6
					</h5>
					<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
						Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
						chronological order.
					</p>
					<Button class="w-fit">
						Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>

				<Card class="m-5">
					<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						CANBUS7
					</h5>
					<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
						Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
						chronological order.
					</p>
					<Button class="w-fit">
						Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>

				<Card class="m-5">
					<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						CANBUS8
					</h5>
					<p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
						Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse
						chronological order.
					</p>
					<Button class="w-fit">
						Read more <ArrowRightOutline class="w-6 h-6 ms-2 text-white" />
					</Button>
				</Card>

				<Card class="m-5">
					<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
						CANBUS9
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
	</div>

	<div>
		<div style="background-color:chartreuse">
			<h1 class="font-semibold text-2xl font-serif">Signals</h1>
		</div>

		<div class="table-wrp block">
			<table class="w-full">
				<thead class="bg-white border-b sticky top-0">
					<tr class="w-full">
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Name
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Multiplexer
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Bit (Start | Size)
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Byte Order
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Value Type
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Scale | Offset
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Min | Max
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Unit
						</th>
						<th scope="col" class="text-sm font-semibold text-gray-900 px-6 py-4 text-left">
							Receiver
						</th>
					</tr>
				</thead>
				<tbody class="overflow-y-auto">
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
					<tr class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100">
						<td class="text-sm text-gray-900 font-light px-6 py-4"> Name </td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Multiplexer</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.0 | 0.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">LittleEndian</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Unsigned</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">0.123 | 123.1</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">-100.0 | 200.0</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">km/h</td>
						<td class="text-sm text-gray-900 font-light px-6 py-4">Recv1, Recv2</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</div>
