use tabled::Tabled;
use crate::decode::DecodedMetar;

pub fn print_table(data: Vec<DecodedMetar>, show_raw: bool) {
    if show_raw {
        #[derive(Tabled)]
        struct Row {
            raw: String,
        }

        let rows: Vec<Row> = data.iter().map(|m| Row {
            raw: m.raw.clone(),
        }).collect();

        let table = tabled::Table::new(rows);
        println!("{}", table);
    } else {
        #[derive(Tabled)]
        struct Row {
            station: String,
            time: String,
            wind: String,
            visibility: String,
            temperature: String,
        }

        let rows: Vec<Row> = data.iter().map(|m| Row {
            station: m.station.clone(),
            time: m.time.clone(),
            wind: m.wind.clone(),
            visibility: m.visibility.clone(),
            temperature: m.temperature.clone(),
        }).collect();

        let table = tabled::Table::new(rows);
        println!("{}", table);
    }
}
