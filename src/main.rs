use std::{fmt::Debug, mem::take};

use rand::{rng, seq::SliceRandom};

fn shuffle_vec_inplace<T>(vec: &mut Vec<T>) {
    vec.shuffle(&mut rng());
}

#[derive(Debug, Clone)]
enum CardType {
    Treasure,
    Action,
    Victory,
    Curse,
}

trait Card: Debug {
    fn name(&self) -> &str;
    fn card_type(&self) -> CardType;
    fn cost(&self) -> u32;
}

#[derive(Debug, Clone)]
enum Treasure {
    Copper,
    Silver,
    Gold,
}

impl Card for Treasure {
    fn name(&self) -> &str {
        match self {
            Treasure::Copper => "Copper",
            Treasure::Silver => "Silver",
            Treasure::Gold => "Gold",
        }
    }

    fn card_type(&self) -> CardType {
        CardType::Treasure
    }

    fn cost(&self) -> u32 {
        match self {
            Treasure::Copper => 0,
            Treasure::Silver => 3,
            Treasure::Gold => 6,
        }
    }
}

#[derive(Debug, Clone)]
enum Victory {
    Estate,
    Duchy,
    Province,
}

impl Card for Victory {
    fn name(&self) -> &str {
        match self {
            Victory::Estate => "Estate",
            Victory::Duchy => "Duchy",
            Victory::Province => "Province",
        }
    }

    fn card_type(&self) -> CardType {
        CardType::Victory
    }

    fn cost(&self) -> u32 {
        match self {
            Victory::Estate => 2,
            Victory::Duchy => 5,
            Victory::Province => 8,
        }
    }
}

#[derive(Debug, Clone)]
enum Curse {
    Curse,
}

impl Card for Curse {
    fn name(&self) -> &str {
        match self {
            Curse::Curse => "Curse",
        }
    }

    fn card_type(&self) -> CardType {
        CardType::Curse
    }

    fn cost(&self) -> u32 {
        match self {
            Curse::Curse => 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Action {
    Moat,
    Village,
    Militia,
    Smithy,
    Remodel,
    Festival,
    Sentry,
    Market,
    Laboratory,
    Artisan,
}

impl Card for Action {
    fn name(&self) -> &str {
        match self {
            Action::Moat => "Moat",
            Action::Village => "Village",
            Action::Militia => "Militia",
            Action::Smithy => "Smithy",
            Action::Remodel => "Remodel",
            Action::Festival => "Festival",
            Action::Sentry => "Sentry",
            Action::Market => "Market",
            Action::Laboratory => "Laboratory",
            Action::Artisan => "Artisan",
        }
    }

    fn card_type(&self) -> CardType {
        CardType::Action
    }

    fn cost(&self) -> u32 {
        match self {
            Action::Moat => 2,
            Action::Village => 3,
            Action::Militia => 4,
            Action::Smithy => 4,
            Action::Remodel => 4,
            Action::Festival => 5,
            Action::Sentry => 5,
            Action::Market => 5,
            Action::Laboratory => 5,
            Action::Artisan => 6,
        }
    }
}

#[derive(Debug)]
struct Player {
    index: usize,
    hand: Vec<Box<dyn Card>>,
    deck: Vec<Box<dyn Card>>,
    discard: Vec<Box<dyn Card>>,
    last_discarded_card: Option<Box<dyn Card>>,
}

impl Player {
    fn new(index: usize) -> Self {
        let mut player = Player {
            index,
            hand: Vec::new(),
            deck: Vec::new(),
            discard: Vec::new(),
            last_discarded_card: None,
        };

        for _ in 0..7 {
            player.deck.push(Box::new(Treasure::Copper));
        }
        for _ in 0..3 {
            player.deck.push(Box::new(Victory::Estate));
        }
        player.shuffle_deck();
        player.draw(5);
        player
    }

    fn draw(&mut self, num_cards_to_draw: usize) {
        if self.deck.len() < num_cards_to_draw {
            self.shuffle_discard();
            self.prepend_discard_to_deck();
        }
        for _ in 0..num_cards_to_draw {
            if let Some(card) = self.deck.pop() {
                self.hand.push(card);
            }
        }
    }

    fn shuffle_deck(&mut self) {
        shuffle_vec_inplace(&mut self.deck);
    }

    fn shuffle_discard(&mut self) {
        shuffle_vec_inplace(&mut self.discard);
    }

    fn prepend_discard_to_deck(&mut self) {
        let mut new_deck = take(&mut self.discard);
        let old_deck = take(&mut self.deck);
        new_deck.extend(old_deck);

        self.deck = new_deck;
    }
}

#[derive(Debug)]
enum Phase {
    Action { curr_player_index: u32 },
    Buy { curr_player_index: u32 },
}

#[derive(Debug)]
struct Supply {
    treasures: Vec<(u32, Box<dyn Card>)>,
    victories: Vec<(u32, Box<dyn Card>)>,
    actions: Vec<(u32, Box<dyn Card>)>,
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    supply: Supply,
    phase: Phase,
}

impl Game {
    fn initialise_game(num_players: usize) -> Self {
        let supply = Supply {
            treasures: vec![
                (60, Box::new(Treasure::Copper)),
                (40, Box::new(Treasure::Silver)),
                (30, Box::new(Treasure::Gold)),
            ],
            victories: vec![
                (8, Box::new(Victory::Estate)),
                (8, Box::new(Victory::Duchy)),
                (8, Box::new(Victory::Province)),
            ],
            actions: vec![
                (10, Box::new(Action::Village)),
                (10, Box::new(Action::Smithy)),
                (10, Box::new(Action::Laboratory)),
            ],
        };

        let mut players = Vec::new();
        for i in 0..num_players {
            players.push(Player::new(i));
        }

        // TODO: Choose this randomly
        let first_player_index = 0;
        let phase = Phase::Action {
            curr_player_index: first_player_index,
        };

        Game {
            players,
            supply,
            phase,
        }
    }
}

fn main() {
    let game = Game::initialise_game(2);
    println!("{:#?}", game);
}
