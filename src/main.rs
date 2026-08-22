slint::include_modules!();
use blackjack_rs::{self, Action, Card, GameState, GameStatus, Rank};
use slint::{Image, ModelRc, VecModel, quit_event_loop};
use std::{cell::RefCell, env, path::PathBuf, rc::Rc, sync::OnceLock};

static ASSETS_DIR: OnceLock<PathBuf> = OnceLock::new();

fn resolve_assets_dir() -> PathBuf {
  let assets_dir_var = "BLACKJACK_ASSETS_DIR";
  if let Ok(path) = env::var(assets_dir_var) {
    PathBuf::from(path)
  } else {
    PathBuf::from("./assets")
  }
}

fn card_image_path(card: Card) -> PathBuf {
  let suit = card.suit.to_string().to_lowercase();
  let raw_rank = card.rank;
  let rank: u8 = match raw_rank {
    Rank::Jack => 11,
    Rank::Queen => 12,
    Rank::King => 13,
    Rank::Ace => 1,
    Rank::Numeric(num) => num,
  };

  let base = ASSETS_DIR.get().unwrap();

  let filename = format!("{}-{}.png", suit, rank);

  base.join("cards").join(filename)
}

fn refresh_ui(app: &AppWindow, game: &GameState) {
  let player_cards: Vec<CardData> = game
    .player_hand()
    .iter()
    .map(|card| CardData {
      image: Image::load_from_path(&card_image_path(*card)).unwrap(),
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
        ASSETS_DIR
          .get()
          .unwrap()
          .join("cards")
          .join("card-mystery.png")
      };
      CardData {
        image: Image::load_from_path(&path).unwrap(),
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
  if round_over {
    app.set_current_screen(Screen::RoundOver);
  }
}

fn main() -> Result<(), slint::PlatformError> {
  ASSETS_DIR.set(resolve_assets_dir()).unwrap();
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
      app.set_current_screen(Screen::Game);
      refresh_ui(&app, &game_handle.borrow());
    });
  }
  {
    app.on_quit(move || {
      let _ = quit_event_loop();
    })
  }
  app.run()
}
