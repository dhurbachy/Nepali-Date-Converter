use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct YearData {
    pub year: i32,
    pub months: Vec<u32>,
    pub total_days: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NepaliDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
     pub day_of_week: Option<u8>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnglishDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}
