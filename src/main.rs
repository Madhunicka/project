use std::vec;
use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]



struct Deck{
    cards: Vec<String>,
}

impl Deck{
    fn new() -> Self{
         //list of suits- 'hearts, spades,clubs,
    //list of values - ace, two three
    //double nested for loop

    let suits =["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two","Three"];
    let mut cards = vec![];


    for suit in suits{

        for value in values{
            let card = format!("{} of {}", value , suit);
            cards.push(card)
            
        }
    }
    Deck{cards}
    

    }

    fn shuffle(&mut self){
        //randommize the order of cards
        //need random number generator(create our own one, intstalling existing ones)
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);

    }

    fn deal(&mut self, num_cards: usize)->Vec<String>{
        self.cards.split_off(self.cards.len()-num_cards)


    }
}


fn main() {
    
   let mut deck = Deck::new();
//    deck.shuffle();
let cards = deck.deal(3);
    
    println!("Heres your deck: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
