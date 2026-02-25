use crate::constant::*;
use crate::types::{EnglishDate, NepaliDate, YearData};
use chrono::{Duration, NaiveDate,Datelike};

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
        let mut days_diff = ad_date.signed_duration_since(epoch).num_days();

        if days_diff < 0 {
            return None;
        }

        for year_info in &self.data {
            // If the days_diff is less than the days in THIS year, the date is HERE.
            if days_diff < year_info.total_days as i64 {
                let mut remaining_days = days_diff;

                for (idx, &days_in_month) in year_info.months.iter().take(12).enumerate() {
                    if remaining_days < days_in_month as i64 {
                        return Some(NepaliDate {
                            year: year_info.year,
                            month: (idx + 1) as u32, // More reliable than manual counter
                            day: (remaining_days + 1) as u32,
                            day_of_week: Some((days_diff % 7) as u8),
                        });
                    }
                    remaining_days -= days_in_month as i64;
                }
            }
            // If we didn't return, subtract this year and move to the next.
            days_diff -= year_info.total_days as i64;
        }
        None
    }

    /// Convert BS to AD
    pub fn bs_to_ad(&self, bs: NepaliDate) -> Option<EnglishDate> {
       let mut total_days: i64 = 0;
    let mut found = false;

    // 1. Accumulate days until we reach the target BS date
    for year_info in &self.data {
        if year_info.year < bs.year {
            total_days += year_info.total_days as i64;
        } else if year_info.year == bs.year {
            // Check if month is valid (1-12)
            if bs.month == 0 || bs.month > 12 { return None; }
            
            for m in 0..(bs.month - 1) {
                total_days += year_info.months[m as usize] as i64;
            }
            found = true;
            break;
        }
    }

    if !found { return None; } // BS Year not in JSON data

    total_days += (bs.day - 1) as i64;

    // 2. Add total days to the AD Epoch
    let epoch = NaiveDate::from_ymd_opt(AD_EPOCH_YEAR, AD_EPOCH_MONTH, AD_EPOCH_DAY)?;
    let ad_naive = epoch + Duration::days(total_days);

    // 3. Map NaiveDate to your EnglishDate struct
    Some(EnglishDate {
        year: ad_naive.year(),
        month: ad_naive.month(),
        day: ad_naive.day(),
    })
    }
}
