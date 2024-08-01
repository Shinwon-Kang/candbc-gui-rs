<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	const dbc_info = {
		file_name: 'CANDBC_FILE_.dbc',
		version: '0.1.0',
		symbols: ['CM_', 'BA_', 'VAL_', 'CAT_'],
		nodes: ['K16_BECM', 'K114B_HPCM', 'T18_BatteryCharger'],
		messages_info: [
			{
				message_name: 'WebData_1840',
				message_cnt: 3
			},
			{
				message_name: 'Battery_Module_1',
				message_cnt: 10
			},
			{
				message_name: 'Battery_Module_2',
				message_cnt: 15
			}
		],
		comments: [
			['Imported file _honda_common.dbc starts here'],
			['BO_', '1840', 'Some Message comment'],
			[
				'SG_',
				'1840',
				'Signal_4',
				'asaklfjlsdfjlsdfglsHH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv'
			],
			['SG_', '5', 'TestSigLittleUnsigned1', '0943503450KFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv'],
			['BU_', 'K17_EBCM', 'Electronic Brake Control Module']
		]
	};
</script>

<div class="space-y-5 text-xl font-semibold dark:text-white">
	<div>
		<p>DBC file:</p>
		<div class="mt-2">
			<Badge variant="secondary" class="text-base">{dbc_info.file_name}</Badge>
		</div>
	</div>
	<div>
		<p>Version:</p>
		<div class="mt-2">
			<Badge variant="secondary" class="mt-1 text-base">{dbc_info.version}</Badge>
		</div>
	</div>
	<div>
		<p>Symbols:</p>
		<div class="mt-2 space-x-2">
			{#each dbc_info.symbols as symbol}
				<Badge variant="secondary" class="text-base">{symbol}</Badge>
			{/each}
		</div>
	</div>
	<div>
		<p>Nodes:</p>
		<div class="mt-2 space-x-1">
			{#each dbc_info.nodes as node}
				<Badge variant="secondary" class="text-base">{node}</Badge>
			{/each}
		</div>
	</div>
	<div>
		<p>Messages:</p>
		<div class="mt-2 space-x-1">
			{#each dbc_info.messages_info as info}
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
			{#each dbc_info.comments as comment}
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
