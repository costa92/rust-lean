extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    #[clap(short, long)]
    name: String,
    #[clap(short, long, default_value = "1")]
    count: u32,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
