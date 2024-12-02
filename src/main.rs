use day_02::find_safe_reports;

fn main() {
    let safe_reports = find_safe_reports("src/resources/puzzle.txt", 0);

    println!("Total safe reports: {safe_reports}");

    let safe_reports = find_safe_reports("src/resources/puzzle.txt", 1);

    println!("Total safe reports with tolerance: {safe_reports}");
}
