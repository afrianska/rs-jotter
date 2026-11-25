#[derive(Debug)]
#[allow(dead_code)]
pub enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CoinNew {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents_new(coin: CoinNew) -> u8 {
    match coin {
        CoinNew::Penny => 1,
        CoinNew::Nickel => 5,
        CoinNew::Dime => 10,
        CoinNew::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
