use peginator::buildscript::Compile;
use std::{env::current_dir, path::PathBuf};

fn main() {
    let path = PathBuf::from(current_dir().unwrap());
    let output = path.join("src/parser/vos.rs");
    Compile::file("src/parser/vos.peg").destination(output).format().run().unwrap();
}
