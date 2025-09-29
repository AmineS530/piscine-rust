#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub products: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            products: vec![],
            receipt: vec![],
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((name, price)) = s
            .products
            .iter()
            .find(|(product_name, _)| *product_name == ele)
        {
            let item = (name.clone(), *price);
            self.products.push(item);
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.products.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let free_n = prices.len() / 3;
        if free_n == 0 {
            self.receipt = prices.clone();
            return prices;
        }

        let total: f32 = prices.iter().sum();
        let free: f32 = prices.iter().take(free_n).sum();
        let disc = total - free;
        let ratio = disc / total;

        let final_p: Vec<f32> = prices
            .into_iter()
            .map(|p| (p * ratio * 100.0).round() / 100.0)
            .collect();

        self.receipt = final_p.clone();
        final_p
    }
}
