use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub ids: String,

    #[arg(long, default_value_t = false)]
    pub raw: bool,
}
