use std::io;

use clap::{self, Parser, Subcommand};

use mdbook_preprocessor::Preprocessor;
use mdbook_trpl::FiguresI18n;

fn main() -> Result<(), String> {
    match Cli::parse().command {
        Some(Command::Supports { renderer }) => {
            if FiguresI18n.supports_renderer(&renderer) {
                Ok(())
            } else {
                Err(format!("Renderer '{renderer}' is unsupported"))
            }
        }
        None => {
            let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())
                .map_err(|e| format!("{e}"))?;
            let processed = FiguresI18n
                .run(&ctx, book)
                .map_err(|e| format!("{e}"))?;
            serde_json::to_writer(io::stdout(), &processed)
                .map_err(|e| format!("{e}"))
        }
    }
}

/// A preprocessor that translates figure captions and image alt text in
/// _The Rust Programming Language_, complementing `mdbook-i18n-helpers` which
/// does not extract these as translatable msgids.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Is the renderer supported?
    Supports { renderer: String },
}
