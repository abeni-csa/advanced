use csv_library::csv_proc::csvreader::{CsvData, read_csv};

fn main() {
    let csv_data: CsvData<i32> = read_csv("input.csv").expect("REASON");
    print!("{:?}", csv_data);
}
