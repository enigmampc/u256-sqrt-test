use primitive_types::U256;

fn main() {
    for x in 1..(1 << 26) {
        let rust = (x as f64).sqrt() as u128;
        let uniswap = u256_sqrt(U256::from(x)).unwrap();

        println!("x       = {}", x);
        if rust != uniswap.low_u128() {
            println!("rust    = {}", rust);
            println!("uniswap = {}", uniswap.low_u128());
        }
        println!("------------------");
    }
}

fn u256_sqrt(y: U256) -> Option<U256> {
    let mut z = U256::from(0);
    if y.gt(&U256::from(3)) {
        z = y.clone();
        let mut x = y.checked_div(U256::from(2))?.checked_add(U256::from(1))?;
        while x.lt(&z) {
            z = x.clone();
            x = y
                .checked_div(x)?
                .checked_add(x)?
                .checked_div(U256::from(2))?;
        }
    } else if !y.is_zero() {
        z = U256::from(1);
    }

    return Some(z);
}
