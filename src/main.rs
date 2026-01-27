mod repl;
mod explorer;
mod binarytree;
mod traits;

use explorer::Explorer;

fn main() {
    let explorer = Explorer::new();
    repl::start(explorer);
}
