pub mod utils;
pub mod types;
pub mod converter;
pub mod constant;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use chrono::NaiveDate;
    use crate::types::{YearData, NepaliDate};
    use crate::converter::DateConverter;

    // Helper function to setup the converter with some sample data
    fn setup_converter() -> DateConverter {
        // 1. Read the JSON file
        // Ensure the path is correct relative to your cargo root
        let data_str = fs::read_to_string("data/calendar_data.json")
            .expect("Could not find calendar_data.json");

        // 2. Parse it into the Vec
        let calendar_data: Vec<YearData> = serde_json::from_str(&data_str)
            .expect("JSON format is incorrect");

        // 3. Return the converter with the real data
        DateConverter::new(calendar_data)
    }

     #[test]
    fn test_ad_to_bs_conversion() {
        let converter = setup_converter();
        
        // Example: Convert 2023-11-29 AD (which is 2080-08-13 BS)
        let english_date = NaiveDate::from_ymd_opt(2023, 11, 29).unwrap();
        
        let result = converter.ad_to_bs(english_date);
        
        assert!(result.is_some(), "Conversion failed");
        let nepali = result.unwrap();
        
        println!("Converted 2023-11-29 to BS: {}-{}-{}", nepali.year, nepali.month, nepali.day);

    }
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
