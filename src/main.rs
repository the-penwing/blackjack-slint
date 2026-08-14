slint::include_modules!();
use blackjack_rs::{self, Action, Card, GameState, GameStatus, Rank};
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

fn refresh_ui(app: &AppWindow, game: &GameState) {
  let player_cards: Vec<CardData> = game
    .player_hand()
    .iter()
    .map(|card| CardData {
      image: Image::load_from_path(Path::new(&card_image_path(*card))).unwrap(),
    })
    .collect();
  let round_over = game.status() != GameStatus::InProgress;
  let dealer_cards: Vec<CardData> = game
    .dealer_hand()
    .iter()
    .enumerate()
    .map(|(i, card)| {
      let path = if i == 0 || round_over {
        card_image_path(*card)
      } else {
        "assets/cards/Unknown_Card.png".to_string()
      };
      CardData {
        image: Image::load_from_path(Path::new(&path)).unwrap(),
      }
    })
    .collect();

  let result_text = match game.status() {
    GameStatus::PlayerBusted => "You Busted!",
    GameStatus::PlayerWon => "You Won!",
    GameStatus::DealerWon => "Dealer Won!",
    GameStatus::Push => "Push! (Draw)",
    GameStatus::InProgress => "",
  };

  let (wins, losses, ties) = game.stats();

  app.set_player_hand(ModelRc::new(VecModel::from(player_cards)));
  app.set_dealer_hand(ModelRc::new(VecModel::from(dealer_cards)));
  app.set_result_text(result_text.into());
  app.set_player_score(game.player_score() as i32);
  app.set_wins(wins as i32);
  app.set_draws(ties as i32);
  app.set_losses(losses as i32);
  app.set_round_over(round_over);
}

fn main() -> Result<(), slint::PlatformError> {
  let app = AppWindow::new()?;

  let game = Rc::new(RefCell::new(GameState::new_game()));
  game.borrow_mut().setup_round();

  refresh_ui(&app, &game.borrow());
  {
    let game_handle = game.clone();
    let app_weak = app.as_weak();
    app.on_hit(move || {
      let app = app_weak.unwrap();
      if game_handle.borrow().status() == GameStatus::InProgress {
        game_handle.borrow_mut().update(Action::Hit);
      }
      refresh_ui(&app, &game_handle.borrow());
    });
  }
  {
    let game_handle = game.clone();
    let app_weak = app.as_weak();
    app.on_stand(move || {
      let app = app_weak.unwrap();
      if game_handle.borrow().status() == GameStatus::InProgress {
        game_handle.borrow_mut().update(Action::Stand);
      }
      refresh_ui(&app, &game_handle.borrow());
    });
  }

  {
    let game_handle = game.clone();
    let app_weak = app.as_weak();
    app.on_new_round(move || {
      let app = app_weak.unwrap();
      game_handle.borrow_mut().setup_round();
      refresh_ui(&app, &game_handle.borrow());
    });
  }
  app.run()
}
