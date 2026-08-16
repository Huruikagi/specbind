use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "specbind",
    version,
    about = "Bind durable specifications to agent-assisted software delivery."
)]
struct Cli {}

fn main() {
    Cli::parse();
}
