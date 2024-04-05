extern crate csv;
extern crate prettytable;

use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;

// Function to prompt the user and read the file path
fn read_file_path() -> Result<String, Box<dyn Error>> {
    println!("Please enter the path to the CSV file:");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    Ok(file_path.trim().to_string())
}

// Function to read CSV data from a file
fn read_csv_data(file_path: &str) -> Result<csv::Reader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let csv_reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    Ok(csv_reader)
}

// Function to list column names in the CLI
fn list_column_names(column_names: &[String]) {
    println!("Column Names:");
    for name in column_names {
        println!("{}", name);
    }
}
// Function to count occurrences of values in one column based on values in another column
fn count_column_by_other_column(
    csv_reader: &mut csv::Reader<File>,
    count_column_index: usize,
    group_by_column_index: usize,
) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for result in csv_reader.records() {
        let record = result?;
        if let Some(count_column_value) = record.get(count_column_index) {
            if let Some(group_by_column_value) = record.get(group_by_column_index) {
                let entry = counts.entry(group_by_column_value.to_string()).or_insert(0);
                *entry += 1;
            }
        }
    }

    Ok(counts)
}
// Function to read CSV data from a file
fn get_headers(csv_reader: &mut csv::Reader<File>) -> Result<Vec<String>, Box<dyn Error>> {
    let headers = csv_reader
        .headers()?
        .iter()
        .map(|h| h.to_string())
        .collect();
    Ok(headers)
}
// Function to create and populate the table
fn create_and_populate_table(csv_reader: &mut csv::Reader<File>) -> Result<Table, Box<dyn Error>> {
    let mut table = Table::new();
    for result in csv_reader.records() {
        let record = result?;
        let mut row = Row::empty();
        for field in record.iter() {
            row.add_cell(Cell::new(field));
        }
        table.add_row(row);
    }
    Ok(table)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read file path
    let file_path = read_file_path()?;

    // Read CSV data
    let mut csv_reader = read_csv_data(&file_path)?;
    let column_names = get_headers(&mut csv_reader)?;
    list_column_names(&column_names);
    // Create and populate the table
    let table = create_and_populate_table(&mut csv_reader)?;

    // Print the table
    table.printstd();

    Ok(())
}
