use lopdf::*;
use pdf_extract::*;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path;
use std::path::PathBuf;

fn main() {
    let file = env::args().nth(1).unwrap();
    println!("Converting {} to text", file);
    let input_path = path::Path::new(&file);
    
    convert_pdf_to_text(input_path);

    //TODO: print the output path
    println!("Conversion complete");

}

fn convert_pdf_to_text(input_path: &std::path::Path) {
    let filename = input_path.file_name().expect("expected a filename");
    
    let mut output_path = PathBuf::new();
    output_path.push("txt");
    output_path.push(filename);
    output_path.set_extension("txt");
    let mut output_file =
    BufWriter::new(File::create(output_path).expect("could not create output"));
    let doc = Document::load(input_path).unwrap();

    print_metadata(&doc);

    let mut output = PlainTextOutput::new(
        &mut output_file as &mut dyn std::io::Write);

    output_doc(&doc, &mut output).unwrap();
}
