// src/formatter.rs
use crate::types::NepaliDate;

pub trait NepaliFormatter {
    fn format(&self, pattern: &str) -> String;
}

impl NepaliFormatter for NepaliDate {
    fn format(&self, pattern: &str) -> String {
        let month_names = [
            "Baisakh", "Jestha", "Ashad", "Shrawan", "Bhadra", "Ashwin",
            "Kartik", "Mangsir", "Poush", "Magh", "Falgun", "Chaitra"
        ];

        let mut result = pattern.to_string();

        // Replace Year
        result = result.replace("YYYY", &self.year.to_string());
        
        // Replace Month (MM for 01, MMMM for Baisakh)
        result = result.replace("MMMM", month_names[(self.month - 1) as usize]);
        result = result.replace("MM", &format!("{:02}", self.month));
        
        // Replace Day (DD)
        result = result.replace("DD", &format!("{:02}", self.day));
        result = result.replace("D", &self.day.to_string());

        result
    }
}
