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
    #[arg(short, long, help = "Tips percentage")]
    tips_percentage: Option<f64>,
}

fn clean_percentage(percentage: f64) -> f64 {
    if percentage > 1.0 {
        percentage / 100.0
    } else {
        percentage
    }
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.file).unwrap();
    let mut products: Vec<Product> = reader::read_file(&file);
    let products = products.iter_mut().collect::<Vec<&mut Product>>();
    let mut invoice = Invoice::new(products);

    if let Some(tips_percentage) = args.tips_percentage {
        let tips_percentage = clean_percentage(tips_percentage);
        println!("Adding tips from products: {}", tips_percentage);
        invoice.tips_from_products(tips_percentage);
    }

    invoice.calculate_taxes();
    invoice.show_invoice(args.show_all);
    invoice.print_resume();
}
