use crate::types::YearData;

pub fn load_calendar_data() -> Vec<YearData> {
    // This anchors the search to your project's root directory
    let json_data = include_str!("../data/calendar_data.json");
    
    serde_json::from_str(json_data).expect("Failed to parse calendar JSON")
}

