use once_cell::sync::Lazy;
use chrono::NaiveDate;
use crate::types::{YearData, NepaliDate};
use crate::converter::DateConverter;

pub mod constant;
pub mod converter;
pub mod types;

// 1. Automatically load and parse the JSON data once at startup
static GLOBAL_CONVERTER: Lazy<DateConverter> = Lazy::new(|| {
    let data_str = include_str!("../data/calendar_data.json");
    let data: Vec<YearData> = serde_json::from_str(data_str)
        .expect("Embedded Nepali calendar data is corrupted");
    DateConverter::new(data)
});

// 2. The "One-Liner" function for your users
pub fn ad_to_bs(date_str: &str) -> Option<NepaliDate> {
    // Parse the input string (YYYY-MM-DD)
    let ad_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
    
    // Call the internal algorithm using the global data
    GLOBAL_CONVERTER.ad_to_bs(ad_date)
}
