use academic_centre::AcademicCentre;
use classroom::Classroom;
use examinee::Examinee;
use serde::{Deserialize, Serialize};
use subject::Subject;
use ts_rs::TS;
use vigilant::Vigilant;

pub mod academic_centre;
pub mod classroom;
pub mod examinee;
pub mod subject;
pub mod vigilant;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, TS)]
#[ts(export, export_to = "../../src/lib/types/generated/")]

pub struct EntityId(i32);

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/types/generated/")]
pub struct AppValues {
    pub examinees: Vec<Examinee>,
    pub academic_centres: Vec<AcademicCentre>,
    pub subjects: Vec<Subject>,
    pub vigilants: Vec<Vigilant>,
    pub classrooms: Vec<Classroom>,
}
