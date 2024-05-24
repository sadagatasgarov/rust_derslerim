#[derive(Debug)]
struct Product {
    name: String,
    price: u32
}



fn main() {
    let laptop = Product {name: "MacBook Pro".to_string(), price: 2000};
    buy_product(laptop, 20);
}



fn buy_product(product: Product, discount: u32) {
    //
}