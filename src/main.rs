use std::{env, fs, process};
use std::fs::File;
use std::path::Path;
use typst_pdf::PdfOptions;
use impress::TypstWrapperWorld;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let typ_file_path = match args.next() {
        Some(path) => path,
        None => {
            println!("No typ file specified.");
            process::exit(1);
        }
    };

    let typ_file = File::open(&typ_file_path).expect("Failed to open typ file");
    let file_name = typ_file_path.split('/').last().unwrap().split('.').nth(0).unwrap();
    let content = std::io::read_to_string(typ_file).expect("Failed to read typ file");

    let root_dir = Path::new(&typ_file_path)
        .parent()
        .unwrap_or(Path::new("."))
        .to_string_lossy()
        .to_string();

    // Create world with content.
    let world = TypstWrapperWorld::new(root_dir.to_owned(), content);

    // Render document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    // Output to pdf
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    fs::write(format!("./{}.pdf", file_name), pdf).expect("Error writing PDF.");
    println!("Created pdf: `./{}.pdf`", file_name);
}