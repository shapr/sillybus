use lopdf::*;
use pdf_extract::*;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path;
use std::path::PathBuf;

fn main() {
    let argument_path = env::args().nth(1).unwrap();
    let input_path = path::Path::new(&argument_path);
    
    if input_path.is_file() {
        println!("File provided");
        convert_pdf_to_text(input_path);
    } else if input_path.is_dir() {
        apply_to_files_in_directory(input_path, &convert_pdf_to_text);
    } else {
        println!("Invalid path provided");
    }
}

fn apply_to_files_in_directory(input_path: &std::path::Path, f: &dyn Fn(&std::path::Path)) {
        match std::fs::read_dir(input_path) {
            Ok(entries) => {
                for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            f(&path);
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
}

fn convert_pdf_to_text(input_path: &std::path::Path) {
    println!("Converting {} to text", input_path.display());

    let filename = input_path.file_name().expect("expected a filename");
    
    let mut output_path = PathBuf::new();
    output_path.push("txt");
    output_path.push(filename);
    output_path.set_extension("txt");
    let mut output_file =
    BufWriter::new(File::create(output_path.clone()).expect("could not create output"));
    let doc = Document::load(input_path).unwrap();

    print_metadata(&doc);

    let mut output = PlainTextOutput::new(
        &mut output_file as &mut dyn std::io::Write);

    output_doc(&doc, &mut output).unwrap();
    println!("Conversion complete at {:?}", output_path);
}
