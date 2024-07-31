<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	import { CircleHelp } from 'lucide-svelte';
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>DRIVER_HEARTBEAT</Card.Title>
		<!-- <Card.Description>Deploy your new project in one-click.</Card.Description> -->
	</Card.Header>
	<Card.Content>
		<form>
			<div class="space-y-1">
				<p class="font-normal text-gray-700 dark:text-white leading-tight">ID: 100</p>
				<p class="font-normal text-gray-700 dark:text-white leading-tight">DLC: 10</p>
				<p class="font-normal text-gray-700 dark:text-white leading-tight">
					Sender: K16_BECM | K16_BECM
				</p>
			</div>
		</form>
	</Card.Content>
	<Card.Footer class="flex justify-between">
		<Button class="w-full text-white place-self-center self-end text-base dark:bg-gray-600">
			View Signals <CircleHelp class="w-5 h-5 ms-2" />
		</Button>
	</Card.Footer>
</Card.Root>
