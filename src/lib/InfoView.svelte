<script lang="ts">
	import { g_selected_dbc, g_dbc_info } from '$lib/stores.js';

	import { Badge } from '$lib/components/ui/badge';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	let selected_dbc = '';
	const sub_selected_dbc = g_selected_dbc.subscribe((dbc) => {
		selected_dbc = dbc;
	});

	let selected_dbc_info = {};
	const sub_selected_dbc_info = g_dbc_info.subscribe((dbc_info) => {
		selected_dbc_info = dbc_info;
	});
</script>

<div class="space-y-5 text-xl font-semibold dark:text-white overflow-y-auto">
	<div>
		<p>DBC file:</p>
		<div class="mt-2">
			<Badge variant="secondary" class="text-base">{selected_dbc_info.file_name}</Badge>
		</div>
	</div>
	{#if selected_dbc_info.version != ''}
		<div>
			<p>Version:</p>
			<div class="mt-2">
				<Badge variant="secondary" class="mt-1 text-base">{selected_dbc_info.version}</Badge>
			</div>
		</div>
	{/if}
	{#if selected_dbc_info.symbols.length > 0}
		<div>
			<p>Symbols:</p>
			<div class="mt-2 flex flex-wrap gap-3">
				{#each selected_dbc_info.symbols as symbol}
					<Badge variant="secondary" class="text-base h-fit w-fit">{symbol}</Badge>
				{/each}
			</div>
		</div>
	{/if}
	{#if selected_dbc_info.nodes.length > 0}
		<div>
			<p>Nodes:</p>
			<div class="mt-2 flex flex-wrap gap-3">
				{#each selected_dbc_info.nodes as node}
					<Badge variant="secondary" class="text-base">{node}</Badge>
				{/each}
			</div>
		</div>
	{/if}
	{#if selected_dbc_info.messages.length > 0}
		<div>
			<p>Messages:</p>
			<div class="mt-2 flex flex-wrap gap-3">
				{#each selected_dbc_info.messages as message}
					<Badge variant="secondary" class="text-base">{message.name} ({message.cnt})</Badge>
				{/each}
			</div>
		</div>
	{/if}
	{#if selected_dbc_info.comments.length > 0}
		<div>
			<p>Comments:</p>

			<ul
				class="mt-2 w-full text-sm font-medium text-gray-900 border border-gray-200 rounded-lg dark:border-gray-600"
			>
				{#each selected_dbc_info.comments as comment}
					<li class="w-full px-4 border-b border-gray-200 rounded-t-lg dark:border-gray-600">
						<div class="py-3 space-x-1">
							{#each comment as c}
								<Badge variant="secondary" class="text-base">{c}</Badge>
							{/each}
						</div>
					</li>
				{/each}
			</ul>
		</div>
	{/if}
</div>
