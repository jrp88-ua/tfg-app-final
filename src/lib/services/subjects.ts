import { Subject, SubjectForCreate, subjectsStore, type SubjectKind } from '$lib/models/subjects';
import type { ModelId } from '$lib/models/models';
import * as m from '$paraglide/messages';
import { get } from 'svelte/store';

let currentId = 0;

export function createSubject(values: SubjectForCreate) {
	const parsed = SubjectForCreate.safeParse(values);
	if (!parsed.success) return false;
	values = parsed.data;

	const subject = new Subject({ id: currentId++, ...values });

	subjectsStore.storeInstance(subject);
	return subject;
}

export function getOrCreateSubject(values: SubjectForCreate) {
	return findSubjectByName(values.name) || createSubject(values);
}

export function findSubjectByName(name: string) {
	return get(getAllSubjects()).find((s) => s.name === name);
}

export function subjectExists(predicate: (subject: Subject) => boolean) {
	return get(getAllSubjects()).find(predicate) !== undefined;
}

export function getAllSubjects() {
	return subjectsStore.getAllInstances();
}

export function getSubject(id: ModelId) {
	return subjectsStore.getInstance(id);
}

export function updatedSubject(id: ModelId) {
	return subjectsStore.updatedInstance(id);
}

export function deleteSubject(id: ModelId) {
	return subjectsStore.deleteInstance(id);
}

export function deleteSubjects(ids: ModelId[]) {
	return subjectsStore.deleteInstances(ids);
}

export function subjectKindValuesTranslate(kind: SubjectKind) {
	switch (kind) {
		case 'OBLIGATORY':
			return m.subject_kind_obligatory();
		case 'VOLUNTARY':
			return m.subject_kind_voluntary();
		case 'UNKNOWN':
			return m.subject_kind_unknown();
	}
}
