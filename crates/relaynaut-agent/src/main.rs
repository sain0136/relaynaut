fn main() {
    println!("Hello, world!");
    let first = 200;
    let sec = 400;
    let mut products: Vec<i32> = Vec::new();
    for n in 1..10 {
        let pro = first * (sec + n - 20);
        products.push(pro);
    }
    println!("Products array:{:?}", products);
}
