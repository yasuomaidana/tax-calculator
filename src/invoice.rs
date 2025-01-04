use crate::product::{
    calculate_total_from_products, calculate_total_from_products_mut, extract_by_type_mut, Product,
};

const VAT: f32 = 0.16;

#[derive(Debug)]
pub struct Invoice<'a> {
    products: Vec<&'a mut Product>,
    tips: Option<Product>,
    taxes: Option<Vec<Product>>,
}

impl<'a> Invoice<'a> {
    pub fn new(raw_products: Vec<&'a mut Product>) -> Self {
        let mut products = raw_products;
        let tips = extract_by_type_mut(&mut products, "Propina").and_then(|mut x| x.pop());
        let taxes = extract_by_type_mut(&mut products, "Impuestos");
        Invoice {
            products,
            tips,
            taxes,
        }
    }

    fn calculate_taxes(&mut self) {
        match self.taxes {
            None => self.calculate_taxes_from_products(),
            Some(_) => {}
        }
    }

    fn total_tips(&self) -> f32 {
        self.tips.as_ref().map_or(0.0, |x| x.price.unwrap_or(0.0))
    }

    fn total_taxes(&self) -> f32 {
        self.taxes
            .as_ref()
            .map_or(0.0, |x| calculate_total_from_products(&x))
    }

    fn total_products(&self) -> f32 {
        calculate_total_from_products_mut(&self.products)
    }

    fn calculate_taxes_from_products(&mut self) {
        let mut base = self.products[0].clone();
        let total = self.remove_vat_from_products();
        base.price = Some(total * VAT);
        self.taxes = Some(vec![base]);
    }

    pub fn calculate_total(&self) -> f32 {
        let total = self.total_products();
        let tips = self.total_tips();
        let taxes = self.total_taxes();
        total + tips + taxes
    }

    fn remove_vat_from_products(&mut self) -> f32 {
        self.products.iter_mut().fold(0.0, |acc, x| {
            let price = x.price.unwrap_or(0.0);
            x.price = Some(price * (1.0 - VAT));
            acc + price
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::invoice::Invoice;
    use crate::product::Product;

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
        assert_eq!(total_products, 6.0*(1.0-VAT));
        assert_eq!(total_tips, 2.0);
        assert_eq!(total_taxes, 6.0*VAT);
    }
}
