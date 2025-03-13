mod blockchain;
mod wallet;
mod network;
mod explorer;
mod token;

fn main() {
    let suzo = token::Token::new();
    println!("Token Name: {}", suzo.name);
    println!("Token Symbol: {}", suzo.symbol);
    println!("Total Supply: {} SUZO", suzo.get_supply());

    network::start_network("8080");  // P2P Network Start
}
