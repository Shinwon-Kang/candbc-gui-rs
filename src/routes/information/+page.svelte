<script lang="ts">
	import InfoView from '$lib/InfoView.svelte';
	import MessageView from '$lib/MessageView.svelte';
	import EmptyComp from '@/EmptyComp.svelte';

	import * as Tabs from '$lib/components/ui/tabs';
	import { Badge } from '$lib/components/ui/badge';

	import { Mail, Info } from 'lucide-svelte';

	let tabs = [
		{ name: 'Information', icon: Info, comp: InfoView },
		{ name: 'Messages', icon: Mail, comp: MessageView }
	];

	let cur = tabs[0];
</script>

<span class="">
	{#each tabs as tab}
		<button
			class:selected={cur === tab}
			on:click={() => (cur = tab)}
			class="relative p-2 px-4 border -mb-px -mt-px -ml-px border-b"
		>
			<span class="flex gap-x-2 mx-2 my-1">
				<svelte:component this={tab.icon} />
				{tab.name}
			</span>
		</button>
	{/each}
</span>

<main class="h-full overflow-y-auto border-t p-5">
	<svelte:component this={cur.comp} />
</main>

<style>
	:global(:root) {
		--color: hsla(0, 0%, 100%, 1);
	}

	:global(:root.dark) {
		--color: hsla(222.2, 84%, 4.9%, 1);
	}

	button.selected {
		border-bottom: 1px solid var(--color);
	}
</style>
