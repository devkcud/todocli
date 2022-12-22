use clap::{ Subcommand, Parser };
use todoman::{ show_todo_list, remove_item, remove_all, toggle_done };
use utils::configuration::load_configuration;

use crate::todoman::add_item;

mod utils;
mod todoman;

#[derive(Subcommand)]
enum Commands {
    /// Add a todo to the list
    Add {
        /// The string to be added. Need to be between quotes (e.g.: "String")
        to_add: String,
    },

    /// Removes a todo to the list
    Remove {
        /// The index of the todo
        index: usize,
    },

    /// Clear the todo list
    Clear {},

    /// Toggle todo 'done' status
    Toggle {
        /// The index of the todo
        index: usize,
    },
}

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();
    let config = load_configuration("todocli");

    match &cli.command {
        Some(Commands::Add { to_add }) => {
            add_item(&config, to_add.as_str(), false);
        }

        Some(Commands::Remove { index }) => {
            remove_item(&config, *index);
        }

        Some(Commands::Clear {}) => {
            remove_all(&config);
        }

        Some(Commands::Toggle { index }) => {
            toggle_done(&config, *index);
        }

        _ => (),
    }

    show_todo_list(&config);
}