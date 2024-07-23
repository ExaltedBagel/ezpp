use std::{io, path::PathBuf};

use clap::Parser;
use clap_derive::Parser;

use ezpp::context::Context;
use ezpp::stream_processor::{self, StreamProcessor};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    context: PathBuf,
}


fn main() {
    let args = Args::parse();
    let context = Context::new_from_yaml(&args.context);
    let mut stdin = io::stdin(); // We get `Stdin` here.
    let mut stdout = io::stdout();

    let processor = StreamProcessor::new(context);
    processor.process_stream(&mut stdin.lock(), &mut stdout.lock());
}
