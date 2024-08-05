<script lang="ts">
	import { g_selected_dbc, g_summary_info } from '$lib/stores.js';

	import { Badge } from '$lib/components/ui/badge';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	let selected_dbc = '';
	const sub_selected_dbc = g_selected_dbc.subscribe((dbc) => {
		selected_dbc = dbc;
	});

	let selected_dbc_summary_info = {};
	const sub_selected_dbc_summary_info= g_summary_info.subscribe((summary_info) => {
		selected_dbc_summary_info = summary_info;
	});

</script>

<div class="space-y-5 text-xl font-semibold dark:text-white">
	<div>
		<p>DBC file:</p>
		<div class="mt-2">
			<Badge variant="secondary" class="text-base">{selected_dbc_summary_info.file_name}</Badge>
		</div>
	</div>
	<div>
		<p>Version:</p>
		<div class="mt-2">
			<Badge variant="secondary" class="mt-1 text-base">{selected_dbc_summary_info.version}</Badge>
		</div>
	</div>
	<div>
		<p>Symbols:</p>
		<div class="mt-2 space-x-2">
			{#each selected_dbc_summary_info.symbols as symbol}
				<Badge variant="secondary" class="text-base">{symbol}</Badge>
			{/each}
		</div>
	</div>
	<div>
		<p>Nodes:</p>
		<div class="mt-2 space-x-1">
			{#each selected_dbc_summary_info.nodes as node}
				<Badge variant="secondary" class="text-base">{node}</Badge>
			{/each}
		</div>
	</div>
	<div>
		<p>Messages:</p>
		<div class="mt-2 space-x-1">
			{#each selected_dbc_summary_info.messages_info as info}
				<Badge variant="secondary" class="text-base">{info.message_name} ({info.message_cnt})</Badge
				>
			{/each}
		</div>
	</div>
	<div>
		<p>Comments:</p>

		<ul
			class="mt-2 w-full text-sm font-medium text-gray-900 border border-gray-200 rounded-lg dark:border-gray-600"
		>
			{#each selected_dbc_summary_info.comments as comment}
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
</div>
