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
    if products.is_empty() { 
        return None
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

fn calculate_total_from_products(products: &[Product]) -> f32 {
    products
        .iter()
        .fold(0.0, |acc, x| acc + x.price.unwrap_or(0.0))
}

fn calculate_taxes_from_products(products: &mut [&mut Product], tips: &Option<Product>) -> Product {
    let mut base = products[0].clone();
    let total = products.iter_mut().fold(0.0, |acc, x| {
        let price = x.price.unwrap_or(0.0);
        x.price = Some(price * 0.84);
        acc + price
    });
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

fn extract_by_type(products: &mut Vec<Product>, product_type: &str) -> Option<Product> {
    let mut to_return = Vec::new();
    products.retain(|product| {
        if product.product_type == product_type {
            to_return.push(product.clone());
            false
        } else {
            true
        }
    });
    reduce_prices(to_return)
}

fn calculate_invoice_total(products: &[Product], tips: &Option<Product>, taxes: &Product) -> f32 {
    let total = calculate_total_from_products(products);
    let total = match tips {
        Some(tips) => match tips.price {
            Some(price) => total + price,
            None => total,
        },
        None => total,
    };
    total + taxes.price.unwrap_or(0.0)
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

    let taxes = extract_by_type(&mut products, "Impuestos");
    let tips = extract_by_type(&mut products, "Propina");
    let mut products = products.iter_mut().collect::<Vec<_>>();

    let taxes = match taxes {
        Some(taxes) => match taxes.price {
            Some(price) => Product {
                date: taxes.date,
                product: taxes.product,
                product_type: taxes.product_type,
                place: taxes.place,
                price: Some(price),
            },
            None => calculate_taxes_from_products(&mut products, &tips),
        },
        None => calculate_taxes_from_products(&mut products, &tips),
    };

    println!("{:?}", products);
    println!("{:?}", tips);
    println!("{:?}", taxes);
    let inmutable_products = products
        .iter_mut()
        .map(|x| x.clone()).collect::<Vec<Product>>();
    println!(
        "Total: {:?}",
        calculate_invoice_total(&inmutable_products, &tips, &taxes)
    );
}
