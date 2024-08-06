<script lang="ts">
	import InfoView from '$lib/InfoView.svelte';
	import MessageView from '$lib/MessageView.svelte';

	import * as Tabs from '$lib/components/ui/tabs';

	import { Badge } from '$lib/components/ui/badge';
</script>

<!-- <div class="bg-green-300">
	<Tabs.Root value="info" class="">
		<Tabs.List class="w-full grid grid-cols-2">
			<Tabs.Trigger value="info">Info</Tabs.Trigger>
			<Tabs.Trigger value="message">Message</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="info">
			<InfoView />
		</Tabs.Content>
		<Tabs.Content value="message">
			<div class="mt-10">
				<MessageView />
			</div>
		</Tabs.Content>
	</Tabs.Root>
</div> -->

<div class="flex flex-col h-full">
	<InfoView />
</div>
