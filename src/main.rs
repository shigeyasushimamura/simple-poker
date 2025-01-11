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

use rand::seq::SliceRandom;
fn main() {
    // let card = Card {
    //     suit: Suit::Club,
    //     rank: 1,
    // };

    // println!("{:?}", card); // デバッグ出力

    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deckを作成
    for suit in suits {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }

    // shuffle
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    println!("{:?}", deck);

    let mut hand: Vec<Card> = Vec::new();

    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}:{:?} {:}", i + 1, card.suit, card.rank);
    }

    // sort
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("---Hand");
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }

    println!("入れ替えたいカードの番号を入力して");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }
}
