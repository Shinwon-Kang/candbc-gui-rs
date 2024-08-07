<script lang="ts">
	import InfoView from '$lib/InfoView.svelte';
	import MessageView from '$lib/MessageView.svelte';
	import EmptyComp from '@/EmptyComp.svelte';

	import * as Tabs from '$lib/components/ui/tabs';
	import { Badge } from '$lib/components/ui/badge';

	import { Mail, Info } from 'lucide-svelte';

	let tabs = [
		{ name: 'Information', icon: Info, comp: EmptyComp },
		{ name: 'Messages', icon: Mail, comp: EmptyComp }
	];

	let cur = tabs[0];
</script>

<span class="">
	{#each tabs as tab}
		<button class:selected={cur === tab} on:click={() => (cur = tab)}>
			<span class="flex gap-x-2 mx-2 my-1">
				<svelte:component this={tab.icon} />
				{tab.name}
			</span>
		</button>
	{/each}
</span>

<main class="bg-orange-300 h-full m-5">
	<svelte:component this={cur.comp} />
</main>

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

<style>
	button {
		cursor: pointer;
		padding: 0.5rem 1rem;
		border: 1px solid transparent;
		position: relative;
		margin-bottom: -1px;
		margin-top: -1px;
		margin-left: -1px;
		background-color: transparent;
		border-bottom: 1px solid hsl(214.3 31.8% 91.4%);
	}
	button.selected {
		border-right: 1px solid hsl(214.3 31.8% 91.4%);
		border-left: 1px solid hsl(214.3 31.8% 91.4%);
		border-bottom: 1px solid #fff;
	}

	main {
		font-size: 1rem;
		padding: 2rem;
		border-top: 1px solid hsl(214.3 31.8% 91.4%);
	}
</style>
