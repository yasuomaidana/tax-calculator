use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(name = "file", help = "File to read")]
    file: String,
}

#[derive(Debug, Clone)]
struct Product {
    date: String,
    product: String,
    product_type: String,
    place: String,
    price: Option<f32>,
}

fn reduce_prices(products: Vec<Product>) -> Option<Product> {
    if products.len() == 0 {
        return None;
    }
    Some(products.iter().fold(products[0].clone(), |acc, x| {
        Product {
            date: acc.date.clone(),
            product: acc.product.clone(),
            product_type: acc.product_type.clone(),
            place: acc.place.clone(),
            price: acc
                .price
                .and_then(|acc_price| x.price.and_then(|x_price| Some(acc_price + x_price))),
        }
    }))
}

fn calculate_total_from_products(products: &Vec<Product>) -> f32 {
    products
        .iter()
        .fold(0.0, |acc, x| acc + x.price.unwrap_or(0.0))
}

fn calculate_taxes_from_products(products: &Vec<Product>, tips: &Option<Product>) -> Product {
    let mut base = products[0].clone();
    let total = calculate_total_from_products(products);
    let total = match tips {
        Some(tips) => match tips.price {
            Some(price) => total - price,
            None => total,
        },
        None => total,
    };
    let calc_taxes = total * 0.16;
    base.price = Some(calc_taxes);
    base
}

fn main() {
    let args = Args::parse();
    let file = std::fs::read_to_string(args.file).unwrap();
    let mut products: Vec<Product> = file
        .split("\n")
        .into_iter()
        .map(|x| x.split("\t"))
        .map(|mut x| {
            let date = x.next().unwrap();
            let product = x.next().unwrap();
            let product_type = x.next().unwrap();
            let place = x.next().unwrap();
            let price = x
                .next()
                .unwrap()
                .replace("$", "")
                .trim()
                .to_owned()
                .parse::<f32>()
                .ok();

            Product {
                date: date.to_owned(),
                product: product.to_owned(),
                product_type: product_type.to_owned(),
                place: place.to_owned(),
                price,
            }
        })
        .collect();
    let mut taxes: Vec<Product> = Vec::new();

    products.retain(|product| {
        if product.product_type == "Impuestos" {
            taxes.push(product.clone());
            false
        } else {
            true
        }
    });

    let mut tips: Vec<Product> = Vec::new();
    products.retain(|product| {
        if product.product_type == "Propina" {
            tips.push(product.clone());
            false
        } else {
            true
        }
    });

    let taxes = reduce_prices(taxes);
    let tips = reduce_prices(tips);

    let taxes = match taxes {
        Some(taxes) => match taxes.price {
            Some(price) => Product {
                date: taxes.date,
                product: taxes.product,
                product_type: taxes.product_type,
                place: taxes.place,
                price: Some(price),
            },
            None => calculate_taxes_from_products(&products, &tips),
        },
        None => calculate_taxes_from_products(&products, &tips),
    };

    println!("{:?}", products);
    println!("{:?}", tips);
    println!("{:?}", taxes);
}
