use super::lookup::LookupInfo;

pub struct OfficialInfo {
    pub id: i32,
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub phone: String,
    pub charge: LookupInfo,
    pub hierarchy_id: LookupInfo,
    pub brigade_id: LookupInfo,
    pub code: Option<i32>,
}
