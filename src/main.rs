extern crate csv;
extern crate prettytable;

use csv::ReaderBuilder;
use prettytable::{Cell, Row, Table};

use std::error::Error;
use std::fs::File;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // Prompt the user to enter the file path
    println!("Please enter the path to the CSV file:");

    // Read the file path from the user input
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;

    // Trim whitespace and remove newline characters
    let file_path = file_path.trim();

    // Open the CSV file
    let file = File::open(file_path)?;

    // Create CSV reader
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Create a table
    let mut table = Table::new();

    // Read and display CSV data
    for result in csv_reader.records() {
        let record = result?;
        let mut row = Row::empty();

        for field in record.iter() {
            row.add_cell(Cell::new(field));
        }

        table.add_row(row);
    }

    // Print the table
    table.printstd();

    Ok(())
}
