use once_cell::sync::Lazy;
use chrono::NaiveDate;
use crate::types::{YearData, NepaliDate,EnglishDate};
use crate::converter::DateConverter;
pub use crate::formatter::NepaliFormatter; 
pub mod constant;
pub mod converter;
pub mod types;
pub mod formatter;

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

pub fn bs_to_ad(date_str:&str)-> Option<EnglishDate> {
// 1. Split the "YYYY-MM-DD" string
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

     // 2. Parse parts into integers and create the NepaliDate struct
    let year: i32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;

        let bs_date = NepaliDate { year, month, day };


    GLOBAL_CONVERTER.bs_to_ad(bs_date)
}
