<script lang="ts">
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Pagination from '$lib/components/ui/pagination';

	const messages = [
		{
			name: 'Name',
			multiplexer: 'Multiplexer',
			bit: '0.0 | 0.1',
			byte_order: 'LittleEndian',
			value_type: 'Unsigned',
			scale_offset: '0.123 | 123.1',
			min_max: '-100.0 | 200.0',
			uint: 'km/h',
			receiver: 'Recv1, Recv2'
		}
	];
</script>

<div>
	<Table.Root>
		<!-- <Table.Caption>A list of your recent invoices.</Table.Caption> -->
		<Table.Header>
			<Table.Row>
				<Table.Head>Name</Table.Head>
				<Table.Head>Multiplexer</Table.Head>
				<Table.Head>Bit (Start | Size)</Table.Head>
				<Table.Head>Byte Order</Table.Head>
				<Table.Head>Value Type</Table.Head>
				<Table.Head>Scale | Offset</Table.Head>
				<Table.Head>Min | Max</Table.Head>
				<Table.Head>Unit</Table.Head>
				<Table.Head>Receiver</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			<!-- {#each messages as message, i (i)} -->
			{#each Array(5) as _, i (i)}
				<Table.Row>
					<Table.Cell>{messages[0].name}</Table.Cell>
					<Table.Cell>{messages[0].multiplexer}</Table.Cell>
					<Table.Cell>{messages[0].bit}</Table.Cell>
					<Table.Cell>{messages[0].byte_order}</Table.Cell>
					<Table.Cell>{messages[0].value_type}</Table.Cell>
					<Table.Cell>{messages[0].scale_offset}</Table.Cell>
					<Table.Cell>{messages[0].min_max}</Table.Cell>
					<Table.Cell>{messages[0].uint}</Table.Cell>
					<Table.Cell>{messages[0].receiver}</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>

	<Pagination.Root count={100} perPage={10} let:pages let:currentPage class="mt-7">
		<Pagination.Content>
			<Pagination.Item>
				<Pagination.PrevButton />
			</Pagination.Item>
			{#each pages as page (page.key)}
				{#if page.type === 'ellipsis'}
					<Pagination.Item>
						<Pagination.Ellipsis />
					</Pagination.Item>
				{:else}
					<Pagination.Item isVisible={currentPage == page.value}>
						<Pagination.Link {page} isActive={currentPage == page.value}>
							{page.value}
						</Pagination.Link>
					</Pagination.Item>
				{/if}
			{/each}
			<Pagination.Item>
				<Pagination.NextButton />
			</Pagination.Item>
		</Pagination.Content>
	</Pagination.Root>
</div>
