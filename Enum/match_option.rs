#[derive(Debug)]
enum Balance {
    Small,
    Intermediate,
    Fish,
    Shark,
}

enum Coin {
    Etherum(Balance),
    Solana(Balance),
    Near(Balance),
    Bitcoin(Balance)
}

fn decimal(coin: Coin) -> u32 {
    match coin {
        Coin::Etherum(bala) => {
            println!("This is a etherum coin {:#?}", bala);
            1
        },
        Coin::Solana(bala) => {
            println!("This is a solana coin {:#?}", bala);
            10
        },
        Coin::Near(bala) => {
            println!("This is a near coin {:#?}", bala);
            100
        },
        Coin::Bitcoin(bala) => {
            println!("This is a bitcoin {:#?}", bala);
            1000
        },
    }

}

fn main() {
    decimal(Coin::Etherum(Balance::Shark));
    decimal(Coin::Solana(Balance::Fish));
    decimal(Coin::Near(Balance::Intermediate));
    decimal(Coin::Bitcoin(Balance::Small));
}
