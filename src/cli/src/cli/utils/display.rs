use tabled::settings::Style;
use tabled::{Table, Tabled};

pub fn print_table<T: Tabled>(data: Vec<T>) {
    let table = Table::new(data).with(Style::modern()).to_string();
    println!("{table}");
}
