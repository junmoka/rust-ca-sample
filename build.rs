use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    pub_controllers();
    pub_usecases();
}

fn dir_list(dir: &str) -> Vec<String> {
    let mut dir_list: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Some(filename) = entry.file_name().to_str() {
                    let name = filename.replace(".rs", "");
                    dir_list.push(name);
                }
            }
        }
    }
    dir_list
}

fn pub_controllers() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("pub_controllers.rs");
    let mut file = File::create(&dest_path).unwrap();
    let controllers = dir_list("src/infra/adapters/controllers");

    write_mod_macro(&mut file, &controllers, "controllers");
    write_use_macro(&mut file, &controllers, "controllers");
}

fn pub_usecases() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("pub_usecases.rs");
    let mut file = File::create(&dest_path).unwrap();
    let usecases = dir_list("src/domain/usecases/");

    write_mod_macro(&mut file, &usecases, "usecases");
    write_use_macro(&mut file, &usecases, "usecases");
}

fn write_mod_macro(file: &mut File, names: &Vec<String>, suffix: &str) {
    file.write(b"macro_rules! ").unwrap();
    file.write(format!("pub_mod_{}", suffix).as_bytes())
        .unwrap();
    file.write(
        b"{
            () => {
    ",
    )
    .unwrap();

    for name in names {
        file.write_fmt(format_args!("pub mod {};\n", name)).unwrap();
    }

    file.write(
        b"
            }
        }",
    )
    .unwrap();
}

fn write_use_macro(file: &mut File, names: &Vec<String>, suffix: &str) {
    file.write(b"macro_rules! ").unwrap();
    file.write(format!("pub_use_{}", suffix).as_bytes())
        .unwrap();
    file.write(
        b"{
            () => {
    ",
    )
    .unwrap();

    for name in names {
        file.write_fmt(format_args!("pub use super::{}::*;\n", name))
            .unwrap();
    }

    file.write(
        b"
            }
        }",
    )
    .unwrap();
}
