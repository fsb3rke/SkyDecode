use tabled::{Table, Tabled};
use crate::decode::DecodedMetar;

#[derive(Tabled)]
pub struct Row {
    station: String,
    time: String,
    wind: String,
    visibility: String,
    temperature: String,
}

pub fn print_table(data: Vec<DecodedMetar>) {
    let rows: Vec<Row> = data.into_iter().map(|m| Row {
        station: m.station,
        time: m.time,
        wind: m.wind,
        visibility: m.visibility,
        temperature: m.temperature,
    }).collect();

    let table = Table::new(rows);
    println!("{table}");
}
