use structopt::StructOpt; // for CliArgs::from_args()

use filter_454;

fn main() {
    let config = filter_454::CliArgs::from_args();

    eprintln!("DEBUG -- {:?}", config);

    filter_454::run(config);
}
