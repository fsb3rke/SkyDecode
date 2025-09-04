mod args;
mod fetch;
mod decode;
mod display;

use args::Args;
use clap::Parser;
use decode::decode_metar;
use display::print_table;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let body = fetch::fetch_metar(&args.ids)
        .await
        .expect("API ERROR");

    let mut decoded_list = vec![];
    for line in body.lines() {
        if let Some(decoded) = decode_metar(line) {
            decoded_list.push(decoded);
        }
    }

    print_table(decoded_list, args.raw);
}
