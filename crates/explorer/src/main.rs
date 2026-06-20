mod repl;
mod explorer;

use explorer::Explorer;

fn main() {
    let explorer = Explorer::new();
    repl::start(explorer);
}
