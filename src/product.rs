#[derive(Debug, Clone)]
pub struct Product {
    pub(crate) date: String,
    pub(crate) product: String,
    pub(crate) product_type: String,
    pub(crate) place: String,
    pub(crate) price: Option<f64>,
}

impl Product {
    pub(crate) fn show(&self) {
        println!("{:0.2}", self.price.unwrap_or(0.0));
    }
    pub(crate) fn show_all(&self) {
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

pub fn extract_by_type_mut(products: &mut Vec<&mut Product>, product_type: &str) -> Option<Vec<Product>> {
    let mut to_return = Vec::new();
    products.retain(|product| {
        if product.product_type == product_type {
            to_return.push((*product).clone());
            false
        } else {
            true
        }
    });
    if to_return.is_empty() {
        None
    } else {
        Some(to_return)
    }
}

pub fn extract_by_type(products: &mut Vec<Product>, product_type: &str) -> Option<Vec<Product>> {
    let mut to_return = Vec::new();
    products.retain(|product| {
        if product.product_type == product_type {
            to_return.push(product.clone());
            false
        } else {
            true
        }
    });
    if to_return.is_empty() {
        None
    } else {
        Some(to_return)
    }
}

pub fn calculate_total_from_products_mut(products: &Vec<&mut Product>) -> f64 {
    products
        .iter()
        .fold(0.0, |acc, x| acc + x.price.unwrap_or(0.0))
}

pub fn calculate_total_from_products(products: &Vec<Product>) -> f64 {
    products
        .iter()
        .fold(0.0, |acc, x| acc + x.price.unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    
    #[test]
    fn test_extract_by_type() {
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
        ];
        let beverages = extract_by_type(&mut products, "Bebida");
        
        assert_eq!(beverages.unwrap().len(), 5);
        assert_eq!(products.len(), 0);
    }
    
    #[test]
    fn test_extract_by_type_mut(){
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
                product: "Cerveza".to_owned(),
                product_type: "Bebida".to_owned(),
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
        let mut products = products.iter_mut().collect::<Vec<_>>();
        let beverages = extract_by_type_mut(&mut products, "Bebida");
        
        assert_eq!(beverages.unwrap().len(), 4);
        assert_eq!(products.len(), 1);
    }
}