use crate::constant::*;
use crate::types::{YearData, NepaliDate, EnglishDate};
use chrono::{NaiveDate, Duration};

pub struct DateConverter {
    data: Vec<YearData>,
}

impl DateConverter {
    pub fn new(data: Vec<YearData>) -> Self {
        Self { data }
    }

    /// Convert AD to BS
    pub fn ad_to_bs(&self, ad_date: NaiveDate) -> Option<NepaliDate> {
        let epoch = NaiveDate::from_ymd_opt(AD_EPOCH_YEAR, AD_EPOCH_MONTH, AD_EPOCH_DAY)?;
        
        // Calculate total days from reference point
        let mut days_diff = ad_date.signed_duration_since(epoch).num_days();

        if days_diff < 0 { return None; } // Date is before year 2000 BS

        for year_info in &self.data {
            if days_diff < year_info.total_days as i64 {
                let mut month = 1;
                let mut remaining_days = days_diff;

                for &days_in_month in &year_info.months {
                    if remaining_days < days_in_month as i64 {
                        return Some(NepaliDate {
                            year: year_info.year,
                            month,
                            day: (remaining_days ) as u32,
                        });
                    }
                    remaining_days -= days_in_month as i64;
                    month += 1;
                }
            }
            days_diff -= year_info.total_days as i64;
        }
        None
    }

    /// Convert BS to AD
    pub fn bs_to_ad(&self, bs: NepaliDate) -> Option<NaiveDate> {
        let mut total_days: i64 = 0;
        
        // 1. Add days for years passed
        for year_info in &self.data {
            if year_info.year < bs.year {
                total_days += year_info.total_days as i64;
            } else if year_info.year == bs.year {
                // 2. Add days for months passed in the target year
                for m in 0..(bs.month - 1) {
                    total_days += year_info.months[m as usize] as i64;
                }
                break;
            }
        }
        
        // 3. Add remaining days
        total_days += (bs.day - 1) as i64;

        let epoch = NaiveDate::from_ymd_opt(AD_EPOCH_YEAR, AD_EPOCH_MONTH, AD_EPOCH_DAY)?;
        Some(epoch + Duration::days(total_days))
    }
}
