mod invoice;
mod product;
mod reader;

use crate::invoice::Invoice;
use crate::product::Product;
use clap::Parser;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(name = "file", help = "File to read")]
    file: PathBuf,
    #[arg(short, long, help = "Show all products", default_value = "false")]
    show_all: bool,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.file).unwrap();
    let mut products: Vec<Product> = reader::read_file(&file);
    let products = products.iter_mut().collect::<Vec<&mut Product>>();
    let mut invoice = Invoice::new(products);
    invoice.calculate_taxes();
    invoice.show_invoice(args.show_all);
    invoice.print_resume();
}
