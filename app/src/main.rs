fn main() {
    
    // call the common functions
    let add = common::add(10, 20);

    let minus = common::utils::minus(20, 10);

    let multiply = common::service::multipy(10, 10);

    println!("add result: {}, minnus: {}, mut: {}",add, minus, multiply);
}
