<script lang="ts">
	import * as Carousel from '$lib/components/ui/carousel/index.js';
	import MessageCard from '$lib/MessageCard.svelte';
	import MessageTable from '$lib/MessageTable.svelte';
	import * as Card from '$lib/components/ui/card/index.js';
</script>

<div>
	<div class="dark:text-white">
		<div class="space-y-2">
			<h1 class="font-semibold text-xl">Messages</h1>
			<p class="mt-1 text-base">
				A packet of data on the CAN bus, containing signals and identified by a unique ID.
			</p>
		</div>
		<div class="mt-7 mx-12 flex justify-center">
			<Carousel.Root class="w-full">
				<Carousel.Content class="-ml-1">
					<!-- If the array size is less than 3, center the card -->
					{#each Array(3) as _, i (i)}
						<Carousel.Item class="pl-1 md:basis-1/2 lg:basis-1/3">
							<div class="p-4">
								<MessageCard />
							</div>
						</Carousel.Item>
					{/each}
				</Carousel.Content>
				<Carousel.Previous />
				<Carousel.Next />
			</Carousel.Root>
		</div>
	</div>
	<div class="mt-14 content-center place-self-end">
		<div class="space-y-2">
			<h1 class="font-semibold text-xl dark:text-white">Signals</h1>
			<p class="mt-1 dark:text-white">
				A specific data point within a message, representing a measurable value like speed or
				temperature.
			</p>
		</div>
		<div class="mt-7">
			<MessageTable />
		</div>
	</div>
</div>
