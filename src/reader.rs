use crate::product::Product;

pub fn read_file(file: &str) -> Vec<Product> {
    file
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
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let file = "2021-01-01\tCerveza\tBebida\tBar\t$2.00\n2021-01-01\tCerveza\tBebida\tBar\t$2.00\n2021-01-01\tCerveza\tBebida\tBar\t$2.00\n2021-01-01\tCerveza\tBebida\tBar\t$2.00\n2021-01-01\tCerveza\tBebida\tBar\t$2.00\n";
        let products = read_file(file);
        assert_eq!(products.len(), 5);
    }
}