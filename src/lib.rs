pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Returns the number of days in a given BS year and month (1-12)
// pub fn get_days_in_month(year: i32, month: usize) -> Option<u8> {
//     // 1. Find the year in the dataset
//     // 2. Return the value at (month - 1) index
//     BS_CALENDAR
//         .iter()
//         .find(|&&(y, _)| y == year)
//         .and_then(|&(_, months)| {
//             if month >= 1 && month <= 12 {
//                 Some(months[month - 1])
//             } else {
//                 None // Invalid month
//             }
//         })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
