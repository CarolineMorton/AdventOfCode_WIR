// Internal imports
use crate::reports::report::check_report;
use crate::reports::dampener::problem_dampener;

/// Check if all reports are safe
///
/// # Arguments
/// * `reports` - The reports to check.
/// * `problem_dampener_on` - A boolean indicating if the problem dampener should be used.
///
/// # Returns
/// Two vectors of reports, one containing the safe reports and the other containing the unsafe reports.
pub fn check_reports(reports: &Vec<Vec<i32>>, problem_dampener_on: bool) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    // Create variables to store the safe and unsafe reports
    let mut safe_reports = Vec::new();
    let mut unsafe_reports = Vec::new();

    if problem_dampener_on {
        // Iterate over the reports
        for report in reports {
            // Check if the reports is safe
            if problem_dampener(report) {
                safe_reports.push(report.clone());
            } else {
                unsafe_reports.push(report.clone());
            }
        }
    } else {

        // Iterate over the reports
        for report in reports {
            // Check if the reports is safe
            if check_report(report) {
                safe_reports.push(report.clone());
            } else {
                unsafe_reports.push(report.clone());
            }
        }
    }

    // Return the safe and unsafe reports
    (safe_reports, unsafe_reports)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_reports_all_safe() {
        // Test all safe reports
        let reports = vec![
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let (safe_reports, unsafe_reports) = check_reports(&reports, false);
        assert_eq!(safe_reports.len(), 3);
        assert_eq!(unsafe_reports.len(), 0);
    }

    #[test]
    fn test_check_reports_all_unsafe() {
        // Test all unsafe reports
        let reports = vec![
            vec![1, 2, 3, 2, 1], // up and then down
            vec![1, 5, 6, 7, 8], // up by more than 3
            vec![9, 7, 6, 2, 1], // down by more than 3
        ];
        let (safe_reports, unsafe_reports) = check_reports(&reports, false);
        assert_eq!(safe_reports.len(), 0);
        assert_eq!(unsafe_reports.len(), 3);
    }

    #[test]
    fn test_check_reports_one_unsafe() {
        // Test one unsafe report
        let reports = vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 2, 1], // up and then down
            vec![5, 4, 3, 2, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let (safe_reports, unsafe_reports) = check_reports(&reports, false);
        assert_eq!(safe_reports.len(), 3);
        assert_eq!(unsafe_reports.len(), 1);

        // Check the unsafe report
        assert_eq!(unsafe_reports[0], vec![1, 2, 3, 2, 1]);
    }

    #[test]
    fn test_check_reports_problem_dampener_all_safe() {
        // Test all safe reports
        let reports = vec![
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let (safe_reports, unsafe_reports) = check_reports(&reports, true);
        assert_eq!(safe_reports.len(), 3);
        assert_eq!(unsafe_reports.len(), 0);
    }

    #[test]
    fn test_check_reports_problem_dampener_all_unsafe() {
        // Test all unsafe reports
        let reports = vec![
            vec![1, 2, 3, 2, 1], // up and then down
            vec![1, 2, 7, 8, 9], // up by more than 3
            vec![9, 7, 6, 2, 1], // down by more than 3
        ];
        let (safe_reports, unsafe_reports) = check_reports(&reports, true);
        assert_eq!(safe_reports.len(), 0);
        assert_eq!(unsafe_reports.len(), 3);
    }

    #[test]
    fn test_check_reports_problem_dampener() {
        // Test example data
        let reports = vec![
            vec![7, 6, 4, 2, 1], // Safe
            vec![1, 2, 7, 8, 9], // Unsafe
            vec![9, 7, 6, 2, 1], // Unsafe
            vec![1, 3, 2, 4, 5], // Unsafe
            vec![8, 6, 4, 4, 1], // Unsafe
            vec![1, 3, 6, 7, 9], // Safe
        ];
        let (safe_reports, unsafe_reports) = check_reports(&reports, false);
        assert_eq!(safe_reports.len(), 2);

        // Check same list with problem dampener on
        let (safe_reports, unsafe_reports) = check_reports(&reports, true);
        assert_eq!(safe_reports.len(), 4);
        assert_eq!(unsafe_reports.len(), 2);
        assert_eq!(unsafe_reports[0], vec![1, 2, 7, 8, 9]);
        assert_eq!(unsafe_reports[1], vec![9, 7, 6, 2, 1]);
    }
}