use crate::product::Product;

pub fn read_file(file: &str) -> Vec<Product> {
    file.split("\n")
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

    #[test]
    fn test_read_file_taxes() {
        let products = "\
        viernes, 27 de diciembre de 2024	Vino Rosado	Alcohol	walmart	 $256.00 
        viernes, 27 de diciembre de 2024	Vino Tinto	Alcohol	walmart	 $148.00 
        viernes, 27 de diciembre de 2024	Sidra	Alcohol	walmart	 $88.45 
        viernes, 27 de diciembre de 2024	Bicarbonato	Abarrotes	walmart	 $24.00 
        viernes, 27 de diciembre de 2024	Pasta	Comida	walmart	 $22.50 
        viernes, 27 de diciembre de 2024	Jeringa	Abarrotes	walmart	 $70.00 
        viernes, 27 de diciembre de 2024	Jabón	Abarrotes	walmart	 $67.00 
        viernes, 27 de diciembre de 2024	Rummy	Ocio	walmart	 $185.00 
        viernes, 27 de diciembre de 2024	Impuestos	Impuestos	walmart	 $157.63 
        ";
        println!("{:?}", read_file(products));
    }
}
