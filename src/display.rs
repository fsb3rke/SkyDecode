use tabled::Tabled;
use tabled::settings::{Style, width::Wrap};
use crate::decode::DecodedMetar;
use terminal_size::{terminal_size, Width};

pub fn print_table(data: Vec<DecodedMetar>, show_raw: bool) {
    let term_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80
    };

    if show_raw {
        #[derive(Tabled)]
        struct Row {
            raw: String,
        }

        let rows: Vec<Row> = data.iter().map(|m| Row {
            raw: m.raw.clone(),
        }).collect();

        let mut table = tabled::Table::new(rows);
        table.with(Style::rounded());
        table.with(Wrap::new(term_width));
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

        let mut table = tabled::Table::new(rows);
        table.with(Style::rounded());
        table.with(Wrap::new(term_width));
        println!("{}", table);
    }
}
