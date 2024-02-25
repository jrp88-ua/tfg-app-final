// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AcademicCentreForExaminee } from './AcademicCentreForExaminee';
import type { ExamineeOrigin } from './ExamineeOrigin';

export interface ExamineeForCreate {
	name: string;
	surenames: string;
	origin: ExamineeOrigin;
	court: number;
	academic_centre: AcademicCentreForExaminee;
}
