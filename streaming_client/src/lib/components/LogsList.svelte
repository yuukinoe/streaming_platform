<script lang="ts">
	import type { LogsTable } from '$lib/types.ts';
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import Loading from '$lib/data/client/components/Loading.svelte';
	const { logsList } = $props<{ logsList: LogsTable[] }>();


	import { Table } from '@flowbite-svelte-plugins/datatable';

	import type { DataTableOptions, ColumnOption } from 'simple-datatables';

	const filterOptions: DataTableOptions = {
		perPageSelect: [5, 10, 15, 20, 25, ['All', -1]],
		rowRender: (row: any, tr: any) => {
			const id = row.cells[0]?.text;
			tr.attributes = {
				...(tr.attributes ?? {}),
				class: `${tr.attributes?.class ?? ''} hover:bg-[#ffffff05] transition-all cursor-pointer`,
				'data-log-id': id
			};
		},
		tableRender: (data: any[], table: any, type: string) => {
			if (type === 'print') {
				return table;
			}

			const tHead = table.childNodes[0];
			const filterHeaders = {
				nodeName: 'TR',
				attributes: {
					class: 'search-filtering-row'
				},
				childNodes: tHead.childNodes[0].childNodes.map((_th: any, index: number) => ({
					nodeName: 'TH',
					childNodes: [
						{
							nodeName: 'INPUT',
							attributes: {
								class: 'datatable-input',
								type: 'search',
								placeholder: `Filter column ${index + 1}`,
								'data-columns': `[${index}]`
							}
						}
					]
				}))
			};

			tHead.childNodes.push(filterHeaders);
			return table;
		}
	};

	onMount(() => {
		const observer = new MutationObserver(() => {
			const trList = document.querySelectorAll('table tbody tr');
			if (!trList.length) return;

			const boundRows = new WeakSet<Element>();

			trList.forEach((tr) => {
				if (boundRows.has(tr)) return;
				boundRows.add(tr);

				const row = tr as HTMLTableRowElement;
				row.addEventListener('click', () => {
					const id = row.dataset.logId;
					if (id) {
						goto(`/admin/administration/logs/${id}`);
					}
				});
			});
		});

		observer.observe(document.body, {
			childList: true,
			subtree: true
		});

		return () => observer.disconnect();
	});
</script>

<div class="flex flex-col gap-5 m-2">
	<div class="flex flex-col text-white">
		<div>
			<span class="text-lg text-[#999999]"> Logs </span>
		</div>
		<div>
			<span class="text-2xl"> View and follow every move on database. </span>
		</div>
		<div>
			<span class="text-lg text-[#999999]"> By viewing this u can easily track every single movement done by any user. </span>
		</div>
	</div>
	<Table items={logsList} dataTableOptions={filterOptions} key={logsList.length} />
</div>
