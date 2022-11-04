use peginator::buildscript::Compile;
use std::env::current_dir;

fn main() {
    let path = current_dir().unwrap();
    let output = path.join("src/parser/tw.rs");
    Compile::file("src/parser/tw.peg").destination(output).format().run().unwrap();
}
