use crate::reports::report::check_report;

/// Problem Dampener
///
/// This iterates through an unsafe report and removes an item (starting from
/// position 0) and checks if the report is safe. If it is, it returns a true
/// (indicating that the report can be made safe and stop iterating. If it is not,
/// it continues to the next item. If a report cannot be made safe, it returns false.
///
/// # Arguments
/// * `report` - The report to check.
///
/// # Returns
/// A boolean indicating if the report can be made safe.
pub fn problem_dampener(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {

        let mut new_report = report.clone();
        new_report.remove(i);
        if check_report(&new_report) {
            return true;
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_dampener_can_be_made_safe() {
        // Test a report that can be made safe. Here we can remove
        // the 3 to make the report safe.
        let report = vec![1, 3, 2, 4, 5];
        assert_eq!(problem_dampener(&report), true);

        // Test a report that can be made safe. Here we can remove
        // the 1 to make the report safe.
        let report = vec![8, 6, 4, 4, 1];
        assert_eq!(problem_dampener(&report), true);
    }

    #[test]
    fn test_problem_dampener_cannot_be_made_safe() {
        // Test a report that cannot be made safe. Here we cannot remove
        // any item to make the report safe.
        let report = vec![1, 2, 7, 8, 9];
        assert_eq!(problem_dampener(&report), false);

        // Test a report that cannot be made safe
        let report = vec![9, 7, 6, 2, 1];
        assert_eq!(problem_dampener(&report), false);
    }
}