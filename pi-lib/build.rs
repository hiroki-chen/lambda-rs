use std::fs;

use lalrpop::Configuration;

fn main() {
    let grammar_files = fs::read_dir("./lang")
        .expect("not a valid path")
        .filter_map(|path| match path {
            Ok(path) => match path.file_name().into_string() {
                Ok(path) => match path.ends_with("lalrpop") {
                    true => Some(format!("lang/{path}")),
                    false => None,
                },
                Err(_) => None,
            },
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    for grammar_file in grammar_files.iter() {
        println!("{}", format!("cargo-rerun-if-changed:lang/{grammar_file}"));

        Configuration::new()
            .use_cargo_dir_conventions()
            .generate_in_source_tree()
            .process_file(grammar_file)
            .unwrap();
    }
}
