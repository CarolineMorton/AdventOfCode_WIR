/// Check if an individual report is safe
///
/// # Arguments
/// * `report` - The report to check.
///
/// # Returns
/// A boolean indicating if the report is safe.
pub fn check_report(report: &Vec<i32>) -> bool {

    // Check if all levels are increasing by 1, 2, or 3
    let mut increasing = true;
    for i in 1..report.len() {
        if report[i] - report[i - 1] > 3 || report[i] <= report[i - 1] {
            increasing = false;
            break;
        }
    }

    let mut decreasing = true;
    for i in 1..report.len() {
        if report[i - 1] - report[i] > 3 || report[i] >= report[i - 1] {
            decreasing = false;
            break;
        }
    }

    increasing != decreasing
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_report_safe() {
        // Test a safe report
        let report = vec![1, 2, 3, 4, 5];
        assert_eq!(check_report(&report), true);

        // Test a different form of a safe report
        let report = vec![5, 4, 3, 2, 1];
        assert_eq!(check_report(&report), true);

        // Test a different form of a safe report
        let report = vec![1, 3, 6, 7, 9];
        assert_eq!(check_report(&report), true);
    }

    #[test]
    fn test_check_report_unsafe() {
        // Test an unsafe report where it goes up and then down
        let report = vec![1, 2, 3, 2, 1];
        assert_eq!(check_report(&report), false);

        // Test an unsafe report where it goes up by more than 3
        let report = vec![1, 5, 6, 7, 8];
        assert_eq!(check_report(&report), false);

        // Test an unsafe report where it goes down by more than 3
        let report = vec![9, 7, 6, 2, 1];
        assert_eq!(check_report(&report), false);
    }

}
