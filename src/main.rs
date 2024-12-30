use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(name = "file", help = "File to read")]
    file: String,
}

#[derive(Debug)]
struct Product {
    date: String,
    product: String,
    product_type: String,
    place: String,
    price: f32,
}

fn main() {
    let args = Args::parse();
    let file = std::fs::read_to_string(args.file).unwrap();
    let products: Vec<Product> = file
        .split("\n")
        .into_iter()
        .map(|x| x.split("\t"))
        .map(|mut x| {
            let date = x.next().unwrap();
            let product = x.next().unwrap();
            let product_type = x.next().unwrap();
            let place = x.next().unwrap();
            let price = x.next().unwrap().replace("$", "")
                .trim().to_owned().parse::<f32>()
                .unwrap_or(-1.0);
            Product {
                date: date.to_owned(),
                product: product.to_owned(),
                product_type: product_type.to_owned(),
                place: place.to_owned(),
                price,
            }
        })
        .collect();
    
    println!("{:?}", products);
}
