// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ExamineeForImport } from './ExamineeForImport';
import type { ExamineeImportColumn } from './ExamineeImportColumn';
import type { ExamineeImportInvalidValueError } from './ExamineeImportInvalidValueError';

export type ExamineeImportError =
	| { type: 'lock' }
	| { type: 'noValuesLoaded' }
	| { type: 'noSheet' }
	| { type: 'missingValue'; row: number; missing: ExamineeImportColumn }
	| {
			type: 'missmatchValue';
			row: number;
			identifier: string;
			missmatch: ExamineeImportColumn;
			establishedValue: string;
			foundValue: string;
	  }
	| {
			type: 'invalidValue';
			row: number;
			reason: ExamineeImportInvalidValueError;
			invalidValue: string;
	  }
	| {
			type: 'missingExamineeValue';
			examinee: ExamineeForImport;
			identifier: string;
			missing: ExamineeImportColumn;
	  };
