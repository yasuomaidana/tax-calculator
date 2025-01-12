use crate::product::{
    calculate_total_from_products, calculate_total_from_products_mut, extract_by_name,
    extract_by_type_mut, Product,
};

const VAT: f64 = 0.16;

#[derive(Debug)]
pub struct Invoice<'a> {
    products: Vec<&'a mut Product>,
    tips: Option<Product>,
    taxes: Option<Vec<Product>>,
}

impl<'a> Invoice<'a> {
    pub fn new(raw_products: Vec<&'a mut Product>) -> Self {
        let mut products = raw_products;
        let tips = extract_by_name(&mut products, "Propina").and_then(|mut x| x.pop());
        let taxes = extract_by_type_mut(&mut products, "Impuestos");
        Invoice {
            products,
            tips,
            taxes,
        }
    }

    pub fn calculate_taxes(&mut self) {
        match self.taxes {
            None => self.calculate_taxes_from_products(),
            Some(_) => self.fix_prices_from_taxes(),
        }
    }

    fn fix_prices_from_taxes(&mut self) {
        let original_products = self
            .products
            .iter_mut()
            .map(|x| x.clone())
            .collect::<Vec<_>>();

        let total_before_vat = self.total_products();
        let total_taxes = self.total_taxes();

        let estimated_ratio = total_taxes / total_before_vat;

        let estimated_taxes = self.products.iter_mut().fold(0.0, |acc, x| {
            let price = x.price.unwrap_or(0.0);
            let price = price * (1.0 - estimated_ratio);
            x.price = Some(price);
            acc + price
        });

        let remaining_taxes = total_before_vat - total_taxes - estimated_taxes;
        let total_products = self.products.len() as f64;
        let estimated_taxes_distribution = remaining_taxes / total_products;

        let products_to_fix = original_products
            .iter()
            .zip(self.products.iter_mut())
            .enumerate()
            .filter_map(|(i, (original, product))| {
                let original_price = original.price.unwrap_or(0.0);
                let price = product.price.unwrap_or(0.0) + estimated_taxes_distribution;
                if price > original_price {
                    None
                } else {
                    Some(i)
                }
            })
            .collect::<Vec<usize>>();

        let products_to_fix_quantity = products_to_fix.len() as f64;
        let remaining_taxes = remaining_taxes / products_to_fix_quantity;

        products_to_fix.iter().for_each(|i| {
            let product = &mut self.products[*i];
            let price = product.price.unwrap_or(0.0) + remaining_taxes;
            product.price = Some(price);
        });
    }

    pub fn total_tips(&self) -> f64 {
        self.tips.as_ref().map_or(0.0, |x| x.price.unwrap_or(0.0))
    }

    pub fn total_taxes(&self) -> f64 {
        self.taxes
            .as_ref()
            .map_or(0.0, |x| calculate_total_from_products(&x))
    }

    pub fn total_products(&self) -> f64 {
        calculate_total_from_products_mut(&self.products)
    }

    fn calculate_taxes_from_products(&mut self) {
        let mut base = self.products[0].clone();
        let total = self.remove_vat_from_products();
        base.product = "IVA".to_owned();
        base.product_type = "Impuestos".to_owned();
        base.price = Some(total * VAT);
        self.taxes = Some(vec![base]);
    }

    pub fn calculate_total(&self) -> f64 {
        let total = self.total_products();
        let tips = self.total_tips();
        let taxes = self.total_taxes();
        total + tips + taxes
    }

    fn remove_vat_from_products(&mut self) -> f64 {
        self.products.iter_mut().fold(0.0, |acc, x| {
            let price = x.price.unwrap_or(0.0);
            x.price = Some(price * (1.0 - VAT));
            acc + price
        })
    }

    pub fn show_invoice(&self, show_all: bool) {
        if show_all {
            self.products.iter().for_each(|x| x.show_all());
            self.tips.as_ref().map(|x| x.show_all());
            self.taxes
                .as_ref()
                .map(|x| x.iter().for_each(|y| y.show_all()));
        } else {
            self.products.iter().for_each(|x| x.show());
            self.tips.as_ref().map(|x| x.show());
            self.taxes
                .as_ref()
                .map(|x| x.iter().for_each(|y| y.show_all()));
        }
    }

    pub fn print_resume(&self) {
        let products_string = format!("Products: ${:0.2}", self.total_products());
        let tips_string = if self.total_tips() > 0.0 {
            format!("Tips: ${:0.2}", self.total_tips())
        } else {
            "".to_owned()
        };
        let taxes_string = format!("Taxes: ${:0.2}", self.total_taxes());
        let total_string = format!("Total: ${:0.2}", self.calculate_total());
        let totals = vec![&products_string, &tips_string, &taxes_string, &total_string];
        let largest_string = find_largest_string(&totals).max(36);
        let header = "=".repeat(largest_string);
        println!("\n{}", header);
        for x in totals {
            if x.is_empty() {
                continue;
            }
            if x.contains("Total") {
                println!("{}", "-".repeat(largest_string));
            }
            let to_fill = largest_string - x.len();
            let left_fill = to_fill / 2 - 1;
            let right_fill = to_fill - left_fill - 2;
            println!("|{}{}{}|", " ".repeat(left_fill), x, " ".repeat(right_fill));
        }
        println!("{}", header);
    }
}

fn find_largest_string(strings: &Vec<&String>) -> usize {
    strings.iter().fold(0, |acc, x| acc.max(x.len())) + 4
}

#[cfg(test)]
mod tests {
    use crate::invoice::{Invoice, VAT};
    use crate::product::Product;
    use crate::reader::read_file;

    #[test]
    fn test_new_invoice() {
        let mut products = vec![
            Product {
                date: "2021-01-01".to_owned(),
                product: "Cerveza".to_owned(),
                product_type: "Bebida".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
            Product {
                date: "2021-01-01".to_owned(),
                product: "Cerveza".to_owned(),
                product_type: "Bebida".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
            Product {
                date: "2021-01-01".to_owned(),
                product: "Cerveza".to_owned(),
                product_type: "Bebida".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
            Product {
                date: "2021-01-01".to_owned(),
                product: "Propina".to_owned(),
                product_type: "Propina".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
            Product {
                date: "2021-01-01".to_owned(),
                product: "IVA".to_owned(),
                product_type: "Impuestos".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
        ];
        let products = products.iter_mut().collect::<Vec<_>>();
        let invoice = Invoice::new(products);
        assert_eq!(invoice.products.len(), 3);
        assert_eq!(invoice.tips.unwrap().price.unwrap(), 2.0);
        assert_eq!(invoice.taxes.unwrap().len(), 1);
    }

    #[test]
    fn test_calculate_taxes_from_restaurant() {
        let mut products = vec![
            Product {
                date: "2021-01-01".to_owned(),
                product: "Cerveza".to_owned(),
                product_type: "Bebida".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
            Product {
                date: "2021-01-01".to_owned(),
                product: "Cerveza".to_owned(),
                product_type: "Bebida".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
            Product {
                date: "2021-01-01".to_owned(),
                product: "Cerveza".to_owned(),
                product_type: "Bebida".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
            Product {
                date: "2021-01-01".to_owned(),
                product: "Propina".to_owned(),
                product_type: "Propina".to_owned(),
                place: "Bar".to_owned(),
                price: Some(2.0),
            },
        ];
        let products = products.iter_mut().collect::<Vec<_>>();
        let mut invoice = Invoice::new(products);
        invoice.calculate_taxes();
        let total = invoice.calculate_total();
        let total_products = invoice.total_products();
        let total_tips = invoice.total_tips();
        let total_taxes = invoice.total_taxes();
        assert_eq!(total, 8.0);
        assert_eq!(total_products, 6.0 * (1.0 - VAT));
        assert_eq!(total_tips, 2.0);
        assert_eq!(total_taxes, 6.0 * VAT);
    }

    #[test]
    fn test_taxes_calculation_from_bill() {
        let raw_invoice = "
        viernes, 27 de diciembre de 2024	Vino Rosado	Alcohol	walmart	 $256.00 
        viernes, 27 de diciembre de 2024	Vino Tinto	Alcohol	walmart	 $148.00 
        viernes, 27 de diciembre de 2024	Sidra	Alcohol	walmart	 $88.45 
        viernes, 27 de diciembre de 2024	Bicarbonato	Abarrotes	walmart	 $24.00 
        viernes, 27 de diciembre de 2024	Pasta	Comida	walmart	 $22.50 
        viernes, 27 de diciembre de 2024	Jeringa	Abarrotes	walmart	 $70.00 
        viernes, 27 de diciembre de 2024	Jabón	Abarrotes	walmart	 $67.00 
        viernes, 27 de diciembre de 2024	Rummy	Ocio	walmart	 $185.00 
        viernes, 27 de diciembre de 2024	IVA	Impuestos	walmart	 $62.10
        viernes, 27 de diciembre de 2024	ISR	Impuestos	walmart	 $95.53 
        ";
        let mut products = read_file(raw_invoice);
        let products = products.iter_mut().collect::<Vec<_>>();
        let mut invoice = Invoice::new(products);
        invoice.calculate_taxes();
        let total = invoice.calculate_total();
        let total_products = invoice.total_products();
        let total_tips = invoice.total_tips();
        let total_taxes = invoice.total_taxes();
        assert!((total - 860.95).abs() < 0.001);
        assert!((total_products - 703.32).abs() < 0.001);
        assert_eq!(total_tips, 0.0);
        assert!((total_taxes - 157.63).abs() < 0.001);
    }

    #[test]
    fn test_taxes_calculation_from_bill_no_taxes() {
        let raw_invoice = "
        viernes, 27 de diciembre de 2024	Vino Rosado	Alcohol	walmart	 $256.00 
        viernes, 27 de diciembre de 2024	Vino Tinto	Alcohol	walmart	 $148.00

        ";
        let mut products = read_file(raw_invoice);
        let products = products.iter_mut().collect::<Vec<_>>();
        let mut invoice = Invoice::new(products);
        invoice.calculate_taxes();
        let total = invoice.calculate_total();
        assert_eq!(total, 404.0);
        let total_products = invoice.total_products();
        let total_tips = invoice.total_tips();
        let total_taxes = invoice.total_taxes();
        assert_eq!(total_products, 404.0 * (1.0 - VAT));
        assert_eq!(total_tips, 0.0);
        assert_eq!(total_taxes, 404.0 * VAT);
    }

    #[test]
    fn test_taxes_calculation_from_restaurant_with_taxes() {
        let raw_invoice = "
        viernes, 27 de diciembre de 2024	Vino Rosado	Alcohol	Restaurant	 $256.00 
        viernes, 27 de diciembre de 2024	Vino Tinto	Alcohol	Restaurant	 $148.00
        viernes, 27 de diciembre de 2024	Propina	Alcohol	Restaurant	 30.00
        viernes, 27 de diciembre de 2024	IVA	Impuestos	walmart	 $20.10
        viernes, 27 de diciembre de 2024	ISR	Impuestos	walmart	 $10.53
        ";
        let mut products = read_file(raw_invoice);
        let products = products.iter_mut().collect::<Vec<_>>();
        let mut invoice = Invoice::new(products);
        invoice.calculate_taxes();
        let total = invoice.calculate_total();
        assert!((total - 434.0).abs() < 0.001);
        let total_products = invoice.total_products();
        let total_tips = invoice.total_tips();
        let total_taxes = invoice.total_taxes();
        assert_eq!(total_products, 373.37);
        assert_eq!(total_tips, 30.0);
        assert!((total_taxes - 30.63).abs() < 0.001);
    }
}
