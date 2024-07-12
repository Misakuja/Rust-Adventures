mod functions;
use functions::GuessingGame;
fn main() {
    let game = GuessingGame::new();
    game.game_logic();
}