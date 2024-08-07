<script lang="ts">
	import * as m from '$paraglide/messages';
	import { DataHandler } from '@vincjo/datatables';
	import ThSort from '$lib/datatable/ThSort.svelte';
	import ThFilter from '$lib/datatable/ThFilter.svelte';
	import Search from '$lib/datatable/Search.svelte';
	import RowsPerPage from '$lib/datatable/RowsPerPage.svelte';
	import RowCount from '$lib/datatable/RowCount.svelte';
	import Pagination from '$lib/datatable/Pagination.svelte';
	import { Examinee } from '$lib/models/examinees';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';
	import type { ModelId } from '$lib/models/models';
	import { deleteExaminees, getAllExaminees } from '$lib/services/examinees';
	import { appState } from '$lib/models/appState';

	import { routeTo } from '$lib/util';
	import { showActionWillDeleteAssignment } from '../actionWillDeleteAssignment';

	const examineesStore = getAllExaminees();
	const modalStore = getModalStore();

	let handler = new DataHandler<Examinee>([], { rowsPerPage: 5 });
	$: handler.setRows($examineesStore);
	const rows = handler.getRows();
	const selected = handler.getSelected();
	const isAllSelected = handler.isAllSelected();

	$: disableDelete = $selected.length === 0;

	async function deleteSelection() {
		if (disableDelete) return;
		if (!(await showActionWillDeleteAssignment(modalStore))) return;
		modalStore.trigger({
			type: 'confirm',
			title: m.delete_examinee_modal_title(),
			body: m.delete_examinee_modal_body({ total: get(selected).length }),
			buttonTextConfirm: m.confirm_delete_selected_examinees(),
			buttonTextCancel: m.cancel_delete_selected_examinees(),
			response: (doDelete: boolean) => {
				if (!doDelete) return;
				const selectedIds = get(selected) as ModelId[];
				deleteExaminees(selectedIds);
				selectedIds.forEach((deleted) => handler.select(deleted));
			}
		});
	}
</script>

<h1 class="text-3xl mb-4">{m.examinees_page_title()}</h1>
<div class="overflow-x-auto space-y-4">
	<!-- Header -->
	<header class="flex justify-between gap-4">
		<div class="flex items-center gap-1">
			<a href="/examinees/create" class="btn variant-filled-primary">
				<span><i class="fa-solid fa-plus" /></span>
				<span>{m.create_examinee()}</span>
			</a>
			<div class="btn-group variant-filled-secondary">
				<a href="/examinees/import">
					<span><i class="fa-solid fa-file-export" /></span>
					<span>{m.import_examinees()}</span>
				</a>
				<!-- <button on:click={() => alert('Aún no se ha implementado')}>
					<span><i class="fa-solid fa-file-import" /></span>
					<span>{m.export_examinees()}</span>
				</button> -->
			</div>
			<button disabled={disableDelete} on:click={deleteSelection} class="btn variant-filled-error">
				<span><i class="fa-solid fa-trash" /></span>
				<span>{m.delete_examinee()}</span>
			</button>
		</div>
		<div class="flex gap-4">
			<Search {handler} />
			<RowsPerPage {handler} />
		</div>
	</header>
	<!-- Table -->
	<table class="table table-hover table-compact w-full table-auto">
		<thead>
			<tr>
				<th class="selection">
					<input
						type="checkbox"
						on:click={() => handler.selectAll({ selectBy: 'id' })}
						checked={$isAllSelected}
					/>
				</th>
				<ThSort {handler} orderBy="nif">{m.examenees_datatable_nif()}</ThSort>
				<ThSort {handler} orderBy="name">{m.examenees_datatable_name()}</ThSort>
				<ThSort {handler} orderBy="surenames">{m.examenees_datatable_surenames()}</ThSort>
				<ThSort {handler} orderBy="origin">{m.examenees_datatable_origin()}</ThSort>
				<ThSort {handler} orderBy="court">{m.examenees_datatable_court()}</ThSort>
				<ThSort {handler} orderBy="lazySubjectsNames">{m.examinees_datatable_subjects()}</ThSort>
				<ThSort {handler} orderBy="lazyAcademicCentreName">
					{m.examenees_datatable_academic_centre()}
				</ThSort>
			</tr>
			<tr>
				<th class="selection" />
				<ThFilter {handler} filterBy="nif" />
				<ThFilter {handler} filterBy="name" />
				<ThFilter {handler} filterBy="surenames" />
				<ThFilter {handler} filterBy="origin" />
				<ThFilter {handler} filterBy="court" />
				<ThFilter {handler} filterBy="lazySubjectsNames" />
				<ThFilter {handler} filterBy="lazyAcademicCentreName" />
			</tr>
		</thead>
		<tbody>
			{#each $rows as row (row.id)}
				<tr
					on:click={(event) => {
						const target = event.target;
						if (!(target instanceof HTMLElement)) return;
						if (
							target.getAttribute('data-row') === 'academic-centre' &&
							row.academicCentreId !== undefined
						) {
							appState.setEdittingAcademicCentre(row.academicCentreId);
							routeTo('/academic-centres/edit');
						} else if (
							target.getAttribute('data-row') === 'subject' &&
							target.getAttribute('data-subject') !== undefined
						) {
							appState.setEdittingSubject(+(target.getAttribute('data-subject') ?? ''));
							routeTo('/subjects/edit');
						} else {
							appState.setEdittingExaminee(row.id);
							routeTo('/examinees/edit');
						}
					}}
				>
					<td class="selection">
						<input
							type="checkbox"
							on:click={(e) => {
								e.stopImmediatePropagation();
								handler.select(row.id);
							}}
							checked={$selected.includes(row.id)}
						/>
					</td>
					<td>{row.nif}</td>
					<td>{row.name}</td>
					<td>{row.surenames}</td>
					<td>{row.origin}</td>
					<td>{row.court}</td>
					<td>
						<ul>
							{#each row.getSubjects() as subject}
								<li data-row="subject" data-subject={subject.id}>{subject.name}</li>
							{/each}
						</ul>
					</td>
					<td data-row="academic-centre">
						{#if row.getAcademicCentre() !== undefined}
							{row.getAcademicCentre()?.name}
						{:else}
							<i>{m.no_academic_centre()}</i>
						{/if}
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<!-- Footer -->
	<footer class="flex justify-between">
		<RowCount {handler} />
		<Pagination {handler} />
	</footer>
</div>
