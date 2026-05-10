use crate::game::{Game, Player};

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_compute_best_move_pass_when_no_resources() {
        let game: Game = "
          oo . oo . oo . oo . oo . oo . oo
          .   10O   .   02W   .   09L   .
     oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   12G   .   06B   .   04W   .   10B   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   .   08O   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   08L   .   03O   .   04G   .   05W   .
     oo W oo W oo . oo . oo . oo . oo . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 0 0 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        assert_eq!(game.compute_best_move(Player::White), "pass");
    }

    #[test]
    fn test_compute_best_move_builds_road_when_affordable() {
        // White has brick=1, lumber=1 → can afford a road; has existing roads to extend from
        let game: Game = "
          oo . oo . oo . oo . oo . oo . oo
          .   10O   .   02W   .   09L   .
     oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   12G   .   06B   .   04W   .   10B   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   .   08O   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   08L   .   03O   .   04G   .   05W   .
     oo W oo W oo . oo . oo . oo . oo . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 1 1 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        let mv = game.compute_best_move(Player::White);
        assert!(mv.starts_with("build_road"), "expected build_road, got: {}", mv);
    }
}

impl Game {
    /// Returns a string describing the best move for `player` given the current game state.
    ///
    /// Priority: build settlement > build road > pass.
    pub fn compute_best_move(&self, player: Player) -> String {
        let resources = match player {
            Player::Red => &self.state.resources.red,
            Player::Blue => &self.state.resources.blue,
            Player::White => &self.state.resources.white,
        };

        // Settlement costs grain + wool + brick + lumber
        if resources.grain >= 1 && resources.wool >= 1
            && resources.brick >= 1 && resources.lumber >= 1
        {
            let spots = self.possible_building_intersections(player);
            if let Some(id) = spots.iter().next() {
                return format!("build_settlement {}", id.0);
            }
        }

        // Road costs brick + lumber
        if resources.brick >= 1 && resources.lumber >= 1 {
            let paths = self.possible_road_paths(player);
            if let Some(path) = paths.iter().next() {
                return format!("build_road {} {}", path.0.0, path.1.0);
            }
        }

        "pass".to_string()
    }
}