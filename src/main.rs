use std::{any::Any, fmt::Debug, mem::take};

use rand::{
    rng,
    seq::{IteratorRandom, SliceRandom},
};

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

    fn as_any(&self) -> &dyn Any;

    fn as_treasure(&self) -> Result<&Treasure, GameError> {
        self.as_any()
            .downcast_ref()
            .ok_or(GameError::FailedToDowncast("Treasure".to_owned()))
    }
    fn as_action(&self) -> Result<&Action, GameError> {
        self.as_any()
            .downcast_ref()
            .ok_or(GameError::FailedToDowncast("Action".to_owned()))
    }
    fn as_victory(&self) -> Result<&Victory, GameError> {
        self.as_any()
            .downcast_ref()
            .ok_or(GameError::FailedToDowncast("Victory".to_owned()))
    }
    fn as_curse(&self) -> Result<&Curse, GameError> {
        self.as_any()
            .downcast_ref()
            .ok_or(GameError::FailedToDowncast("Curse".to_owned()))
    }
}

#[derive(Debug, Clone)]
enum Treasure {
    Copper,
    Silver,
    Gold,
}

impl Treasure {
    fn value(&self) -> u32 {
        match self {
            Treasure::Copper => 1,
            Treasure::Silver => 2,
            Treasure::Gold => 3,
        }
    }
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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Player {
    index: usize,
    hand: Vec<Box<dyn Card>>,
    deck: Vec<Box<dyn Card>>,
    discard: Vec<Box<dyn Card>>,
    played: Vec<Box<dyn Card>>,
    trashed: Vec<Box<dyn Card>>,
    last_discarded_card: Option<Box<dyn Card>>,
    actions: u32,
    buys: u32,
    coins: u32,
}

impl Player {
    fn new(index: usize) -> Self {
        let mut player = Player {
            index,
            hand: Vec::new(),
            deck: Vec::new(),
            discard: Vec::new(),
            played: Vec::new(),
            trashed: Vec::new(),
            last_discarded_card: None,
            actions: 1,
            buys: 1,
            coins: 0,
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

    fn get_victory_points(&self) -> u32 {
        self.hand
            .iter()
            .chain(self.deck.iter())
            .chain(self.discard.iter())
            .fold(0, |sum, card| {
                if card.name() == "Estate" {
                    sum + 1
                } else if card.name() == "Duchy" {
                    sum + 3
                } else if card.name() == "Province" {
                    sum + 6
                } else {
                    sum
                }
            })
    }

    fn remove_card_from_hand(&mut self, card_index: usize) -> Result<Box<dyn Card>, GameError> {
        if card_index >= self.hand.len() {
            Err(GameError::CardNotFound("Index out of bounds".to_owned()))
        } else {
            Ok(self.hand.remove(card_index))
        }
    }

    fn play_card(&mut self, card: Box<dyn Card>) {
        self.hand.push(card);
    }
}

#[derive(Debug)]
struct Supply {
    treasures: Vec<(u32, Box<dyn Card>)>,
    victories: Vec<(u32, Box<dyn Card>)>,
    actions: Vec<(u32, Box<dyn Card>)>,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Card not found in hand: {0}")]
    CardNotFound(String),

    #[error("Not enough money: required {required}, had {available}")]
    NotEnoughMoney { required: u32, available: u32 },

    #[error("Invalid move: {0}")]
    InvalidMove(String),

    #[error("Failed to downcast card type: {0}")]
    FailedToDowncast(String),

    #[error("Supply pile empty: {0}")]
    EmptySupply(String),
}

#[derive(Debug)]
enum GameMove {
    PlayCard {
        curr_player_index: usize,
        card_index: usize,
    },
    BuyCard {
        player_index: usize,
        card: Box<dyn Card>,
    },
    DiscardCard {
        player_index: usize,
        card: Box<dyn Card>,
    },
    EndActions {
        player_index: usize,
    },
    EndTurn {
        player_index: usize,
    },
}

#[derive(Debug)]
enum GamePhase {
    // Regular turn phases
    ActionPhase,
    BuyPhase,
    CleanupPhase,
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    supply: Supply,
    curr_player_index: usize,
    game_phase: GamePhase,
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
                (10, Box::new(Action::Moat)),
                (10, Box::new(Action::Village)),
                (10, Box::new(Action::Militia)),
                (10, Box::new(Action::Smithy)),
                (10, Box::new(Action::Remodel)),
                (10, Box::new(Action::Festival)),
                (10, Box::new(Action::Sentry)),
                (10, Box::new(Action::Market)),
                (10, Box::new(Action::Laboratory)),
                (10, Box::new(Action::Artisan)),
            ],
        };

        Game {
            players: (0..num_players).map(|i| Player::new(i)).collect(),
            supply,
            curr_player_index: (0..num_players).choose(&mut rng()).unwrap(),
            game_phase: GamePhase::ActionPhase,
        }
    }

    fn accept_move(&mut self, game_move: GameMove) -> Result<(), GameError> {
        match (&self.game_phase, game_move) {
            (
                GamePhase::ActionPhase,
                GameMove::PlayCard {
                    curr_player_index,
                    card_index,
                },
            ) => {
                if curr_player_index != self.curr_player_index {
                    return Err(GameError::InvalidMove("Wrong player index".to_owned()));
                }
                let card_to_play =
                    self.players[self.curr_player_index].remove_card_from_hand(card_index)?;
                match card_to_play.card_type() {
                    CardType::Treasure => {
                        self.end_actions()?;
                        self.players[self.curr_player_index].coins +=
                            card_to_play.as_treasure()?.value();
                        self.players[self.curr_player_index].play_card(card_to_play);
                        Ok(())
                    }
                    CardType::Action => todo!(),
                    CardType::Victory => Err(GameError::InvalidMove(
                        "Cannot play victory card".to_owned(),
                    )),
                    CardType::Curse => Err(GameError::InvalidMove("Cannot play curse".to_owned())),
                }
            }
            _ => Err(GameError::InvalidMove("Invalid move".to_owned())),
        }
    }

    fn end_actions(&mut self) -> Result<(), GameError> {
        if let GamePhase::ActionPhase = self.game_phase {
            self.game_phase = GamePhase::BuyPhase;
            Ok(())
        } else {
            Err(GameError::InvalidMove("Not in action phase".to_owned()))
        }
    }
}

fn main() {
    let game = Game::initialise_game(2);
    println!("{:#?}", game);
}
