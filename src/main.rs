slint::include_modules!();
use blackjack_rs::{self, Card, GameState, Rank};
use slint::{Image, ModelRc, VecModel};
use std::{cell::RefCell, path::Path, rc::Rc};

fn card_image_path(card: Card) -> String {
  let suit = card.suit.to_string();
  let raw_rank = card.rank;
  let rank: u8 = match raw_rank {
    Rank::Jack => 11,
    Rank::Queen => 12,
    Rank::King => 13,
    Rank::Ace => 1,
    Rank::Numeric(num) => num,
  };
  format!("assets/cards/{}_{}.png", suit, rank)
}

fn main() -> Result<(), slint::PlatformError> {
  let app = AppWindow::new()?;

  let game = Rc::new(RefCell::new(GameState::new_game()));
  game.borrow_mut().setup_round();

  let player_cards: Vec<CardData> = game
    .borrow()
    .player_hand()
    .iter()
    .map(|card| CardData {
      image: Image::load_from_path(Path::new(&card_image_path(*card))).unwrap(),
    })
    .collect();
  let dealer_cards: Vec<CardData> = game
    .borrow()
    .dealer_hand()
    .iter()
    .enumerate()
    .map(|(i, card)| {
      let path = if i == 0 {
        card_image_path(*card)
      } else {
        "assets/cards/Unknown_Card.png".to_string()
      };
      CardData {
        image: Image::load_from_path(Path::new(&path)).unwrap(),
      }
    })
    .collect();

  app.set_player_hand(ModelRc::new(VecModel::from(player_cards)));
  app.set_dealer_hand(ModelRc::new(VecModel::from(dealer_cards)));
  app.run()
}
