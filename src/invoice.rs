use crate::product::{
    calculate_total_from_products, calculate_total_from_products_mut, extract_by_type_mut, Product,
};

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

    fn calculate_taxes_from_products(&mut self) {
        let mut base = self.products[0].clone();
        let total = self.calculate_products_taxes();
        base.price = Some(total * 0.16);
        self.taxes = Some(vec![base]);
    }

    pub fn calculate_total(&self) -> f32 {
        let total = calculate_total_from_products_mut(&self.products);
        let tips = self
            .tips
            .as_ref()
            .map(|x| x.price.unwrap_or(0.0))
            .unwrap_or(0.0);
        let taxes = self
            .taxes
            .as_ref()
            .map_or(0.0, |x| calculate_total_from_products(&x));
        total + tips + taxes
    }
    
    fn calculate_products_taxes(&mut self) -> f32 {
        self.products.iter_mut().fold(0.0, |acc, x| {
            let price = x.price.unwrap_or(0.0);
            x.price = Some(price * 0.84);
            acc + price
        })
    }
}

#[cfg(test)]
mod tests{
    use crate::invoice::Invoice;
    use crate::product::Product;

    #[test]
    fn test_new_invoice(){
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
}