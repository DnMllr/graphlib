use clap::Parser;

mod gen;
mod pathfind;

/// Simple program to generate a directed graph
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
enum Program {
    Gen {
        /// The number of nodes in the graph
        #[clap(short, long)]
        count: usize,

        /// The probability of two nodes having an edge
        #[clap(short, long)]
        probability: f64,
    },

    Pathfind {
        /// The starting point
        #[clap(short, long)]
        start: usize,

        /// The ending point
        #[clap(short, long)]
        end: usize,
    },
}

impl Program {
    pub fn run(self) -> color_eyre::Result<()> {
        match self {
            Self::Gen { count, probability } => gen::print_graph(count, probability, false),
            Self::Pathfind { start, end } => pathfind::pathfind(start, end),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Program::parse().run()
}
