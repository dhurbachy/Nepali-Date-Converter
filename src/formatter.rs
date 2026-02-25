// src/formatter.rs
use crate::types::NepaliDate;

pub trait NepaliFormatter {
    fn format(&self, pattern: &str) -> String;
}

impl NepaliFormatter for NepaliDate {
    fn format(&self, pattern: &str) -> String {
        let month_names = [
            "Baisakh", "Jestha", "Ashad", "Shrawan", "Bhadra", "Ashwin", "Kartik", "Mangsir",
            "Poush", "Magh", "Falgun", "Chaitra",
        ];

        // Assuming your day_of_week logic is calculated in your conversion step
        let day_names_en = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        let day_names_np = [
            "आइतबार",  // Aaitabar
            "सोमबार",  // Sombar
            "मंगलबार",  // Mangalbar
            "बुधबार",   // Budhabar
            "बिहीबार", // Bihibar
            "शुक्रबार",  // Sukrabar
            "शनिबार",  // Sanibar
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

        // If you don't have day_of_week in your struct yet, you'll need to calculate it
        if let Some(dw) = self.day_of_week {
            result = result.replace("EEEE", day_names_en[dw as usize]);
            result = result.replace("NNNN", day_names_np[dw as usize]);
        }

        result
    }
}
