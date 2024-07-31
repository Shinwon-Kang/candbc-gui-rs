<script lang="ts">
	import SummaryView from '$lib/SummaryView.svelte';
	import MessageView from '$lib/MessageView.svelte';

	import * as Tabs from '$lib/components/ui/tabs';
</script>

<div class="h-full m-5">
	<Tabs.Root value="summary" class="mt-5 mx-5">
		<Tabs.List class="grid w-full grid-cols-2">
			<Tabs.Trigger value="summary">Summary</Tabs.Trigger>
			<Tabs.Trigger value="message">Message</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="summary">
			<div class="mt-10">
				<SummaryView />
			</div>
		</Tabs.Content>
		<Tabs.Content value="message">
			<div class="mt-10">
				<MessageView />
			</div>
		</Tabs.Content>
	</Tabs.Root>
</div>
