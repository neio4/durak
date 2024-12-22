use durak::game;

fn main() {
    println!("Hello, world!");
    let card = game::Card::new(game::Suit::Hearts, game::Suit::Clubs, game::Rank::Two);
    let card2 = game::Card::new(game::Suit::Clubs, game::Suit::Clubs, game::Rank::Ace);
    println!("{:?}", card2);
    println!("{:?}", card);
}
