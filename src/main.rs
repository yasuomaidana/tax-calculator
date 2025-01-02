use clap::Parser;
use std::fmt::Debug;

#[derive(Parser, Debug)]
struct Args {
    #[arg(name = "file", help = "File to read")]
    file: String,
    #[arg(short, long, help = "Show all products", default_value = "false")]
    show_all: bool,
}

#[derive(Debug, Clone)]
struct Product {
    date: String,
    product: String,
    product_type: String,
    place: String,
    price: Option<f32>,
}

impl Product {
    fn show(&self) {
        println!("{:0.2}", self.price.unwrap_or(0.0));
    }
    fn show_all(&self) {
        println!(
            "{} {:?} {} {:?} {:0.2}",
            self.date,
            self.product,
            self.product_type,
            self.place,
            self.price.unwrap_or(0.0)
        );
    }
}

fn reduce_prices(products: Vec<Product>) -> Option<Product> {
    if products.is_empty() {
        return None;
    }
    let total = calculate_total_from_products(&products);
    let mut base = products[0].clone();
    base.price = Some(total);
    Some(base)
}

fn calculate_total_from_products(products: &[Product]) -> f32 {
    products
        .iter()
        .fold(0.0, |acc, x| acc + x.price.unwrap_or(0.0))
}

fn calculate_prices_from_taxes(products: &mut [&mut Product], taxes: &Product) {
    let original_products = products
        .iter_mut()
        .map(|x| x.clone())
        .collect::<Vec<Product>>();

    let total = calculate_products_total_mut(products);
    let taxes = taxes.price.unwrap_or(0.0);
    let ratio = taxes / total;
    let first_taxes = products.iter_mut().fold(0.0, |acc, x| {
        let price = x.price.unwrap_or(0.0);
        let price = price * (1.0 - ratio);
        x.price = Some(price);
        acc + price
    });
    let remaining_taxes = total - first_taxes - taxes;
    let total_products = products.len() as f32;

    let to_fix = original_products
        .iter()
        .zip(products.iter_mut())
        .enumerate()
        .filter_map(|(i, (original, product))| {
            let original_price = original.price.unwrap_or(0.0);
            let price = product.price.unwrap_or(0.0) + remaining_taxes / total_products;
            if price > original_price {
                None
            } else {
                Some(i)
            }
        })
        .collect::<Vec<usize>>();

    let to_fix_quantity = to_fix.len() as f32;
    let remaining_taxes = remaining_taxes / to_fix_quantity;
    to_fix.iter().for_each(|i| {
        let product = &mut products[*i];
        let price = product.price.unwrap_or(0.0) + remaining_taxes;
        product.price = Some(price);
    });
}

fn calculate_products_total_mut(products: &mut [&mut Product]) -> f32 {
    products.iter_mut().fold(0.0, |acc, x| {
        let price = x.price.unwrap_or(0.0);
        x.price = Some(price * 0.84);
        acc + price
    })
}

fn calculate_taxes_from_products(products: &mut [&mut Product], tips: &Option<Product>) -> Product {
    let mut base = products[0].clone();
    let total = calculate_products_total_mut(products);
    let total = match tips {
        Some(tips) => match tips.price {
            Some(price) => total - price,
            None => total,
        },
        None => total,
    };
    let calc_taxes = total * 0.16;
    base.product_type = "Impuestos".to_owned();
    base.product = "Impuestos".to_owned();
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

fn show_final_invoice(
    products: &[Product],
    tips: &Option<Product>,
    taxes: &Product,
    show_all: bool,
) {
    if show_all {
        products.iter().for_each(|product| {
            product.show_all();
        });
        match tips {
            Some(tips) => tips.show_all(),
            None => {}
        }
        taxes.show_all();
    } else {
        products.iter().for_each(|product| {
            product.show();
        });
        match tips {
            Some(tips) => tips.show(),
            None => {}
        }
        taxes.show();
    }
}
fn main() {
    let args = Args::parse();
    let file = std::fs::read_to_string(args.file).unwrap();
    let mut products: Vec<Product> = file
        .split("\n")
        .into_iter()
        .map(|x| x.split("\t"))
        .filter(|x| x.clone().count() >= 5)
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
            Some(_price) => {
                calculate_prices_from_taxes(&mut products, &taxes);
                taxes
            }
            None => calculate_taxes_from_products(&mut products, &tips),
        },
        None => calculate_taxes_from_products(&mut products, &tips),
    };

    let inmutable_products = products
        .iter_mut()
        .map(|x| x.clone())
        .collect::<Vec<Product>>();
    show_final_invoice(&inmutable_products, &tips, &taxes, args.show_all);
    println!(
        "Total: {:?}",
        calculate_invoice_total(&inmutable_products, &tips, &taxes)
    );
}
