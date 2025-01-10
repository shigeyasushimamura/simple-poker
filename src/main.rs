#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug)] // Card構造体にもDebugトレイトを付与
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let card = Card {
        suit: Suit::Club,
        rank: 1,
    };

    println!("{:?}", card); // デバッグ出力
}
