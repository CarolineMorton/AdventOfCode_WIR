/// Load reports from file
///
/// # Arguments
/// * `filepath` - The file path to the reports file.
///
/// # Returns
/// A vector of vectors of integers representing the reports.
pub fn load_reports(filepath: &str) -> Vec<Vec<i32>> {
    // Create a variable to store the reports
    let mut reports = Vec::new();

    // Read the file
    let file = std::fs::read_to_string(filepath).expect("Unable to read file");

    // Iterate over the lines in the file
    for line in file.lines() {
        // Create a vector to store the levels in the reports
        let mut report = Vec::new();

        // Iterate over the levels in the reports
        for level in line.split_whitespace() {
            // Parse the level as an integer
            let level = level.parse().expect("Unable to parse level");

            // Add the level to the reports
            report.push(level);
        }

        // Add the reports to the reports
        reports.push(report);
    }

    // Return the reports
    reports
}
