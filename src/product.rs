#[derive(Debug, Clone)]
pub struct Product {
    pub(crate) date: String,
    pub(crate) product: String,
    pub(crate) product_type: String,
    pub(crate) place: String,
    pub(crate) price: Option<f32>,
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