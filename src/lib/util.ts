import * as m from '$paraglide/messages';

export function createSheetColumns(amount: number): string[] {
	const columns: string[] = [];
	for (let i = 0; i < amount; i++)
		columns.push(m.sheet_column_name({ letter: sheetColumnLetter(i) }));

	return columns;
}

export function sheetColumnLetter(n: number): string {
	const ordA = 'A'.charCodeAt(0);
	const ordZ = 'Z'.charCodeAt(0);
	const len = ordZ - ordA + 1;

	let s = '';
	while (n >= 0) {
		s = String.fromCharCode((n % len) + ordA) + s;
		n = Math.floor(n / len) - 1;
	}
	return s;
}