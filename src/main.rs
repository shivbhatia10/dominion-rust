use std::{any::Any, collections::HashMap, fmt::Debug, mem::take};

use rand::{
    rng,
    seq::{IteratorRandom, SliceRandom},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Card not found in hand: {0}")]
    CardNotFound(String),

    #[error("Card not found in supply: {0}")]
    CardNotFoundInSupply(String),

    #[error("Card supply depleted: {0}")]
    CardSupplyDepleted(String),

    #[error("Not enough money: required {required}, had {available}")]
    NotEnoughMoney { required: u32, available: u32 },

    #[error("Invalid move: {0}")]
    InvalidMove(String),

    #[error("Failed to downcast card type: {0}")]
    FailedToDowncast(String),

    #[error("Supply pile empty: {0}")]
    EmptySupply(String),
}

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

    fn end_turn(&mut self) {
        self.discard_hand();
        self.clear_played();
        self.actions = 1;
        self.buys = 1;
        self.coins = 0;
        self.draw(5);
    }

    fn discard_hand(&mut self) {
        let hand = take(&mut self.hand);
        self.discard.extend(hand);
    }

    fn clear_played(&mut self) {
        let played = take(&mut self.played);
        self.discard.extend(played);
    }
}

#[derive(Debug)]
struct Supply {
    // Maps from card name to quantity
    treasures: HashMap<String, u8>,
    actions: HashMap<String, u8>,
    victories: HashMap<String, u8>,
    curses: HashMap<String, u8>,
}

impl Supply {
    fn take_card(&mut self, card_to_take: Box<dyn Card>) -> Result<(), GameError> {
        match card_to_take.card_type() {
            CardType::Treasure => {
                Supply::take_from_supply_pile(&mut self.treasures, card_to_take.name())
            }
            CardType::Victory => {
                Supply::take_from_supply_pile(&mut self.victories, card_to_take.name())
            }
            CardType::Action => {
                Supply::take_from_supply_pile(&mut self.actions, card_to_take.name())
            }
            CardType::Curse => Supply::take_from_supply_pile(&mut self.curses, card_to_take.name()),
        }
    }

    fn take_from_supply_pile(
        pile: &mut HashMap<String, u8>,
        card_name: &str,
    ) -> Result<(), GameError> {
        if let Some(count) = pile.get_mut(card_name) {
            if *count <= 0 {
                Err(GameError::CardSupplyDepleted(card_name.to_owned()))
            } else {
                *count -= 1;
                Ok(())
            }
        } else {
            Err(GameError::CardNotFoundInSupply(card_name.to_owned()))
        }
    }
}

#[derive(Debug)]
enum GameMove {
    PlayCard { card_index: usize },
    BuyCard { card: Box<dyn Card> },
    DiscardCard { card: Box<dyn Card> },
    EndActions,
    EndTreasures,
    EndTurn,
}

#[derive(Debug)]
enum GamePhase {
    // Regular turn phases
    ActionPhase,
    TreasurePhase,
    BuyPhase,
    // Will add more for special phases
}

struct Game {
    players: Vec<Player>,
    supply: Supply,
    curr_player_index: usize,
    game_phase: GamePhase,
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Current player index: {:?}\nGame phase: {:?}\nSupply: {:#?}\nCurrent player hand: {:#?}",
            self.curr_player_index, self.game_phase, self.supply, self.current_player_read_only().hand
        )
    }
}

impl Game {
    fn initialise_game(num_players: usize) -> Self {
        use Action::*;
        use Treasure::*;
        use Victory::*;
        let supply = Supply {
            treasures: HashMap::from([
                (Copper.name().to_owned(), 60),
                (Silver.name().to_owned(), 40),
                (Gold.name().to_owned(), 30),
            ]),
            actions: HashMap::from([
                (Moat.name().to_owned(), 10),
                (Village.name().to_owned(), 10),
                (Militia.name().to_owned(), 10),
                (Smithy.name().to_owned(), 10),
                (Remodel.name().to_owned(), 10),
                (Festival.name().to_owned(), 10),
                (Sentry.name().to_owned(), 10),
                (Market.name().to_owned(), 10),
                (Laboratory.name().to_owned(), 10),
                (Artisan.name().to_owned(), 10),
            ]),
            victories: HashMap::from([
                (Province.name().to_owned(), 10),
                (Duchy.name().to_owned(), 10),
                (Estate.name().to_owned(), 10),
            ]),
            curses: HashMap::from([(Curse::Curse.name().to_owned(), 10)]),
        };

        Game {
            players: (0..num_players).map(|i| Player::new(i)).collect(),
            supply,
            curr_player_index: (0..num_players).choose(&mut rng()).unwrap(),
            game_phase: GamePhase::ActionPhase,
        }
    }

    fn current_player(&mut self) -> &mut Player {
        &mut self.players[self.curr_player_index]
    }

    fn current_player_read_only(&self) -> &Player {
        &self.players[self.curr_player_index]
    }

    fn accept_move(&mut self, player_index: usize, game_move: GameMove) -> Result<(), GameError> {
        if player_index != self.curr_player_index {
            return Err(GameError::InvalidMove("Wrong player index".to_owned()));
        }
        match (&self.game_phase, game_move) {
            // ACTION PHASE
            (GamePhase::ActionPhase, GameMove::PlayCard { card_index }) => {
                let card_to_play = self.current_player().remove_card_from_hand(card_index)?;
                match card_to_play.card_type() {
                    CardType::Treasure => Err(GameError::InvalidMove(
                        "Cannot play treasure in action phase".to_owned(),
                    )),
                    CardType::Action => {
                        if self.current_player_read_only().actions == 0 {
                            return Err(GameError::InvalidMove("No actions left".to_owned()));
                        }
                        self.current_player().actions -= 1;
                        let action = card_to_play.as_action()?;
                        self.current_player().play_card(Box::new(action.clone()));

                        self.handle_action(action)?;

                        if self.current_player_read_only().actions == 0 {
                            self.action_to_treasure_phase()?
                        }
                        Ok(())
                    }
                    CardType::Victory => Err(GameError::InvalidMove(
                        "Cannot play victory card".to_owned(),
                    )),
                    CardType::Curse => Err(GameError::InvalidMove("Cannot play curse".to_owned())),
                }
            }
            (GamePhase::ActionPhase, GameMove::EndActions) => {
                self.current_player().actions = 0;
                self.action_to_treasure_phase()
            }

            // TREASURE PHASE
            (GamePhase::TreasurePhase, GameMove::PlayCard { card_index }) => {
                let card_to_play = self.current_player().remove_card_from_hand(card_index)?;
                match card_to_play.card_type() {
                    CardType::Treasure => {
                        self.current_player().coins += card_to_play.as_treasure()?.value();
                        self.current_player().play_card(card_to_play);
                        Ok(())
                    }
                    CardType::Action => Err(GameError::InvalidMove(
                        "Cannot play action card in treasure phase".to_owned(),
                    )),
                    CardType::Victory => Err(GameError::InvalidMove(
                        "Cannot play victory card".to_owned(),
                    )),
                    CardType::Curse => Err(GameError::InvalidMove("Cannot play curse".to_owned())),
                }
            }
            (GamePhase::TreasurePhase, GameMove::EndTreasures) => self.treasure_to_buy_phase(),

            // BUY PHASE
            (GamePhase::BuyPhase, GameMove::BuyCard { card }) => {
                let cost = card.cost();
                if self.current_player_read_only().coins < cost {
                    return Err(GameError::NotEnoughMoney {
                        required: cost,
                        available: self.current_player_read_only().coins,
                    });
                }
                self.current_player().coins -= cost;
                self.supply.take_card(card)?;
                self.current_player().buys -= 1;
                if self.current_player_read_only().buys == 0 {
                    self.end_turn()?
                }
                Ok(())
            }

            (_, GameMove::EndTurn) => {
                self.end_turn()?;
                Ok(())
            }

            _ => Err(GameError::InvalidMove(
                "Invalid move for given game phase".to_owned(),
            )),
        }
    }

    fn handle_action(&mut self, action: &Action) -> Result<(), GameError> {
        match action {
            Action::Moat => {
                self.current_player().draw(2);
            }
            Action::Village => {
                self.current_player().actions += 2;
                self.current_player().draw(1);
            }
            Action::Militia => todo!(),
            Action::Smithy => {
                self.current_player().draw(3);
            }
            Action::Remodel => todo!(),
            Action::Festival => {
                self.current_player().actions += 2;
                self.current_player().buys += 1;
            }
            Action::Sentry => todo!(),
            Action::Market => {
                self.current_player().buys += 1;
                self.current_player().actions += 1;
                self.current_player().draw(1);
            }
            Action::Laboratory => {
                self.current_player().actions += 1;
                self.current_player().draw(2);
            }
            Action::Artisan => todo!(),
        }
        Ok(())
    }

    // PHASE TRANSITIONS
    fn action_to_treasure_phase(&mut self) -> Result<(), GameError> {
        if let GamePhase::ActionPhase = self.game_phase {
            self.game_phase = GamePhase::TreasurePhase;
            Ok(())
        } else {
            Err(GameError::InvalidMove(
                "Not in action phase, cannot enter treasure phase".to_owned(),
            ))
        }
    }
    fn treasure_to_buy_phase(&mut self) -> Result<(), GameError> {
        if let GamePhase::TreasurePhase = self.game_phase {
            self.game_phase = GamePhase::BuyPhase;
            Ok(())
        } else {
            Err(GameError::InvalidMove(
                "Not in treasure phase, cannot enter buy phase".to_owned(),
            ))
        }
    }
    fn end_turn(&mut self) -> Result<(), GameError> {
        self.current_player().end_turn();
        self.curr_player_index = (self.curr_player_index + 1) % self.players.len();
        self.game_phase = GamePhase::ActionPhase;
        Ok(())
    }
}

fn main() -> Result<(), GameError> {
    let mut game = Game::initialise_game(2);
    println!("{:#?}", game);
    game.accept_move(0, GameMove::PlayCard { card_index: 0 })?;
    println!("{:#?}", game);
    Ok(())
}
