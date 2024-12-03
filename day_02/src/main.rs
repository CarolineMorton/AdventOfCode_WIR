// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a reports only counts as safe if both of the following are true:
//
// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// In the example above, the reports can be found safe or unsafe by checking those rules:
//
// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

// External imports
use clap::Parser;

// Internal imports
mod args_parser;
mod loader;
mod reports;
mod reports_checker;

use args_parser::Args;
use loader::load_reports;

use reports_checker::check_reports;





fn main() {
    println!("Welcome to Day 2!");

    // Parse command line arguments
    let args = Args::parse();

    // Load the reports from the file
    let reports = load_reports(&args.file);

    // Check the reports without problem dampener on
    let (safe_reports, _) = check_reports(&reports, false);

    // Print the safe reports length with problem dampener off
    println!("Number of safe reports without problem dampener: {}", safe_reports.len());

    // Check the reports with problem dampener on
    let (safe_reports, _) = check_reports(&reports, true);

    // Print the safe reports length with problem dampener on
    println!("Number of safe reports with problem dampener: {}", safe_reports.len());
}
