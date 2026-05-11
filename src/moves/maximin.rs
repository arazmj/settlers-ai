use crate::game::{Building, BuildingKind, Game, IntersectionId, Path, PathId, Player, Road, State};
use crate::game::resources::{ResourceCount, CITY_COST, ROAD_COST, SETTLEMENT_COST};
use std::fmt;

/// Search depth for the minimax tree.  Each level = one player's turn.
/// White → Red → Blue counts as 3 plies; DEPTH=3 looks 1 full round ahead.
const DEPTH: u32 = 3;

/// A single action a player can take on their turn.
#[derive(Debug, Clone)]
pub(crate) enum GameMove {
    BuildRoad(Path),
    BuildSettlement(IntersectionId),
    BuildCity(IntersectionId),
    Pass,
}

impl fmt::Display for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameMove::BuildRoad(path) => write!(f, "build_road {} {}", path.0.0, path.1.0),
            GameMove::BuildSettlement(id) => write!(f, "build_settlement {}", id.0),
            GameMove::BuildCity(id) => write!(f, "build_city {}", id.0),
            GameMove::Pass => write!(f, "pass"),
        }
    }
}

fn next_player(player: Player) -> Player {
    match player {
        Player::Red   => Player::Blue,
        Player::Blue  => Player::White,
        Player::White => Player::Red,
    }
}

fn deduct_resources(state: &mut State, player: Player, cost: ResourceCount) {
    match player {
        Player::Red   => state.resources.red   = state.resources.red.clone()   - cost,
        Player::Blue  => state.resources.blue  = state.resources.blue.clone()  - cost,
        Player::White => state.resources.white = state.resources.white.clone() - cost,
    }
}

impl Game {
    fn player_resources(&self, player: Player) -> &ResourceCount {
        match player {
            Player::Red   => &self.state.resources.red,
            Player::Blue  => &self.state.resources.blue,
            Player::White => &self.state.resources.white,
        }
    }

    fn victory_points(&self, player: Player) -> u32 {
        let building_vp: u32 = self.state.buildings.iter()
            .filter(|b| b.player == player)
            .map(|b| match b.kind {
                BuildingKind::Settlement => 1,
                BuildingKind::City => 2,
            })
            .sum();
        let road_bonus = if self.longest_road(player) >= 5 { 2 } else { 0 };
        building_vp + road_bonus
    }

    /// Expected resource units per turn: sum of dice probabilities of tiles adjacent to
    /// player buildings, doubled for cities.
    fn resource_income(&self, player: Player) -> u32 {
        self.state.buildings.iter()
            .filter(|b| b.player == player)
            .map(|b| {
                let mult = match b.kind {
                    BuildingKind::Settlement => 1,
                    BuildingKind::City => 2,
                };
                self.board.intersection_dice_income(b.intersection_id.0) * mult
            })
            .sum()
    }

    /// Heuristic score for `player`.
    ///
    /// Primary driver: victory points (×100).  Road progress toward the longest-road bonus
    /// earns proportional credit (×3 per road, 200 at the threshold).  Resource income
    /// serves as a tiebreaker to prefer better-placed settlements.
    fn score(&self, player: Player) -> i32 {
        let road_len = self.longest_road(player) as i32;
        let road_score = if road_len >= 5 { 200 } else { road_len * 3 };
        self.victory_points(player) as i32 * 100 + road_score + self.resource_income(player) as i32
    }

    fn generate_moves(&self, player: Player) -> Vec<GameMove> {
        let res = self.player_resources(player);
        let mut moves = vec![GameMove::Pass];

        if res.brick >= 1 && res.lumber >= 1 {
            for path in self.possible_road_paths(player) {
                moves.push(GameMove::BuildRoad(path));
            }
        }

        if res.grain >= 1 && res.wool >= 1 && res.brick >= 1 && res.lumber >= 1 {
            for id in self.possible_building_intersections(player) {
                moves.push(GameMove::BuildSettlement(id));
            }
        }

        if res.grain >= 2 && res.ore >= 3 {
            for b in self.state.buildings.iter()
                .filter(|b| b.player == player && matches!(b.kind, BuildingKind::Settlement))
            {
                moves.push(GameMove::BuildCity(b.intersection_id));
            }
        }

        moves
    }

    fn apply_move(&self, mv: &GameMove, player: Player) -> Game {
        let mut g = self.clone();
        match mv {
            GameMove::BuildRoad(path) => {
                let path_id = g.board.paths.iter().position(|p| p == path)
                    .expect("path not found in board");
                g.state.roads.push(Road { id: PathId(path_id), player });
                deduct_resources(&mut g.state, player, ROAD_COST);
            }
            GameMove::BuildSettlement(id) => {
                g.state.buildings.push(Building {
                    intersection_id: *id,
                    kind: BuildingKind::Settlement,
                    player,
                });
                deduct_resources(&mut g.state, player, SETTLEMENT_COST);
            }
            GameMove::BuildCity(id) => {
                if let Some(b) = g.state.buildings.iter_mut()
                    .find(|b| b.intersection_id == *id && b.player == player)
                {
                    b.kind = BuildingKind::City;
                }
                deduct_resources(&mut g.state, player, CITY_COST);
            }
            GameMove::Pass => {}
        }
        g
    }

    /// Paranoid minimax: `maximizing_player` maximises their score; every other player
    /// acts as a minimiser (assumes the worst case for us).
    fn minimax(&self, depth: u32, player: Player, maximizing_player: Player) -> i32 {
        if depth == 0 {
            return self.score(maximizing_player);
        }
        let moves = self.generate_moves(player);
        let next  = next_player(player);
        if player == maximizing_player {
            moves.iter()
                .map(|mv| self.apply_move(mv, player).minimax(depth - 1, next, maximizing_player))
                .max()
                .unwrap_or(i32::MIN)
        } else {
            moves.iter()
                .map(|mv| self.apply_move(mv, player).minimax(depth - 1, next, maximizing_player))
                .min()
                .unwrap_or(i32::MAX)
        }
    }

    /// Returns a string describing the best move for `player` given the current game state.
    pub fn compute_best_move(&self, player: Player) -> String {
        let moves = self.generate_moves(player);
        let next  = next_player(player);
        moves.iter()
            .map(|mv| {
                let score = self.apply_move(mv, player).minimax(DEPTH - 1, next, player);
                (score, mv)
            })
            .max_by_key(|(score, _)| *score)
            .map(|(_, mv)| mv.to_string())
            .unwrap_or_else(|| "pass".to_string())
    }
}

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
        // White has brick=1, lumber=1 and two existing roads to extend from.
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

    #[test]
    fn test_compute_best_move_prefers_settlement_over_road() {
        // Settlement (+1 VP) beats building another road in the scoring function.
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
W 1 1 1 1 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        let mv = game.compute_best_move(Player::White);
        assert!(mv.starts_with("build_settlement"), "expected build_settlement, got: {}", mv);
    }

    #[test]
    fn test_compute_best_move_city_upgrade() {
        // White settlement at intersection 39, grain=2 + ore=3 → should upgrade to city.
        let game: Game = "
          oo . oo . oo . oo . oo . oo . oo
          .   10O   .   02W   .   09L   .
     oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   12G   .   06B   .   04W   .   10B   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   .   08O   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   08L   .   03O   .   04G   .   05W   .
     oo W WS W oo . oo . oo . oo . oo . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 2 0 0 0 3
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        let mv = game.compute_best_move(Player::White);
        assert!(mv.starts_with("build_city"), "expected build_city, got: {}", mv);
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    use crate::game::{Board, RobberId, Tile, TileKind};

    fn zero_rc() -> ResourceCount {
        ResourceCount { grain: 0, wool: 0, brick: 0, lumber: 0, ore: 0 }
    }

    /// Builds a Game with the standard test tile set, the given pieces, and White's resources.
    /// Red and Blue always start at zero resources.
    fn make_game(buildings: Vec<Building>, roads: Vec<Road>, white: ResourceCount) -> Game {
        let tiles = [
            Tile { dice: 10, kind: TileKind::Ore },    Tile { dice: 2,  kind: TileKind::Wool },
            Tile { dice: 9,  kind: TileKind::Lumber }, Tile { dice: 12, kind: TileKind::Grain },
            Tile { dice: 6,  kind: TileKind::Brick },  Tile { dice: 4,  kind: TileKind::Wool },
            Tile { dice: 10, kind: TileKind::Brick },  Tile { dice: 9,  kind: TileKind::Grain },
            Tile { dice: 11, kind: TileKind::Lumber }, Tile { dice: 0,  kind: TileKind::Nothing },
            Tile { dice: 3,  kind: TileKind::Lumber }, Tile { dice: 8,  kind: TileKind::Ore },
            Tile { dice: 8,  kind: TileKind::Lumber }, Tile { dice: 3,  kind: TileKind::Ore },
            Tile { dice: 4,  kind: TileKind::Grain },  Tile { dice: 5,  kind: TileKind::Wool },
            Tile { dice: 5,  kind: TileKind::Brick },  Tile { dice: 6,  kind: TileKind::Grain },
            Tile { dice: 11, kind: TileKind::Wool },
        ];
        use crate::game::resources::PlayerResourceCount;
        Game {
            board: Board::new(tiles),
            state: State {
                buildings,
                roads,
                robber: RobberId(0),
                resources: PlayerResourceCount { red: zero_rc(), blue: zero_rc(), white },
            },
        }
    }

    // ── victory_points ────────────────────────────────────────────────────────

    #[test]
    fn test_victory_points_no_buildings() {
        let game = make_game(vec![], vec![], zero_rc());
        assert_eq!(game.victory_points(Player::White), 0);
    }

    #[test]
    fn test_victory_points_one_settlement() {
        let b = vec![Building { intersection_id: IntersectionId(0), kind: BuildingKind::Settlement, player: Player::White }];
        assert_eq!(make_game(b, vec![], zero_rc()).victory_points(Player::White), 1);
    }

    #[test]
    fn test_victory_points_city_worth_two() {
        let b = vec![Building { intersection_id: IntersectionId(0), kind: BuildingKind::City, player: Player::White }];
        assert_eq!(make_game(b, vec![], zero_rc()).victory_points(Player::White), 2);
    }

    #[test]
    fn test_victory_points_mixed_buildings() {
        // 1 settlement + 1 city = 3 VP.
        let b = vec![
            Building { intersection_id: IntersectionId(0), kind: BuildingKind::Settlement, player: Player::White },
            Building { intersection_id: IntersectionId(3), kind: BuildingKind::City,       player: Player::White },
        ];
        assert_eq!(make_game(b, vec![], zero_rc()).victory_points(Player::White), 3);
    }

    #[test]
    fn test_victory_points_longest_road_bonus() {
        // Paths 54–58 form a 5-road chain (38→39→40→41→42→43) → +2 VP bonus.
        let roads: Vec<Road> = (54..=58)
            .map(|id| Road { id: PathId(id), player: Player::White })
            .collect();
        let game = make_game(vec![], roads, zero_rc());
        assert_eq!(game.longest_road(Player::White), 5);
        assert_eq!(game.victory_points(Player::White), 2);
    }

    #[test]
    fn test_victory_points_only_own_buildings_counted() {
        // Red buildings must not inflate White's VP count.
        let b = vec![Building { intersection_id: IntersectionId(0), kind: BuildingKind::City, player: Player::Red }];
        assert_eq!(make_game(b, vec![], zero_rc()).victory_points(Player::White), 0);
    }

    // ── resource_income ───────────────────────────────────────────────────────

    #[test]
    fn test_resource_income_no_buildings() {
        assert_eq!(make_game(vec![], vec![], zero_rc()).resource_income(Player::White), 0);
    }

    #[test]
    fn test_resource_income_settlement() {
        // Intersection 10 is adjacent to tiles 0 (dice=10→3), 1 (dice=2→1), 4 (dice=6→5).
        // Expected income = 3 + 1 + 5 = 9.
        let b = vec![Building { intersection_id: IntersectionId(10), kind: BuildingKind::Settlement, player: Player::White }];
        assert_eq!(make_game(b, vec![], zero_rc()).resource_income(Player::White), 9);
    }

    #[test]
    fn test_resource_income_city_doubles() {
        // Same intersection but City: 9 × 2 = 18.
        let b = vec![Building { intersection_id: IntersectionId(10), kind: BuildingKind::City, player: Player::White }];
        assert_eq!(make_game(b, vec![], zero_rc()).resource_income(Player::White), 18);
    }

    // ── apply_move ────────────────────────────────────────────────────────────

    #[test]
    fn test_apply_move_pass_leaves_state_unchanged() {
        let game = make_game(vec![], vec![], zero_rc());
        let after = game.apply_move(&GameMove::Pass, Player::White);
        assert_eq!(after.state.roads.len(), 0);
        assert_eq!(after.state.buildings.len(), 0);
    }

    #[test]
    fn test_apply_move_road_adds_road_and_deducts_resources() {
        // White has roads at 54 (38–39) and 55 (39–40); path 56 (40–41) is free.
        let roads = vec![
            Road { id: PathId(54), player: Player::White },
            Road { id: PathId(55), player: Player::White },
        ];
        let white = ResourceCount { grain: 0, wool: 0, brick: 1, lumber: 1, ore: 0 };
        let game  = make_game(vec![], roads, white);
        let after = game.apply_move(&GameMove::BuildRoad(Path(IntersectionId(40), IntersectionId(41))), Player::White);
        assert_eq!(after.state.roads.len(), 3);
        assert_eq!(after.state.resources.white.brick,  0);
        assert_eq!(after.state.resources.white.lumber, 0);
    }

    #[test]
    fn test_apply_move_settlement_adds_building_and_deducts_resources() {
        let roads = vec![
            Road { id: PathId(54), player: Player::White },
            Road { id: PathId(55), player: Player::White },
        ];
        let white = ResourceCount { grain: 1, wool: 1, brick: 1, lumber: 1, ore: 0 };
        let game  = make_game(vec![], roads, white);
        let after = game.apply_move(&GameMove::BuildSettlement(IntersectionId(38)), Player::White);
        assert_eq!(after.state.buildings.len(), 1);
        assert!(matches!(after.state.buildings[0].kind, BuildingKind::Settlement));
        assert_eq!(after.state.resources.white.grain,  0);
        assert_eq!(after.state.resources.white.wool,   0);
        assert_eq!(after.state.resources.white.brick,  0);
        assert_eq!(after.state.resources.white.lumber, 0);
    }

    #[test]
    fn test_apply_move_city_upgrades_kind_and_deducts_resources() {
        let b = vec![Building { intersection_id: IntersectionId(10), kind: BuildingKind::Settlement, player: Player::White }];
        let white = ResourceCount { grain: 2, wool: 0, brick: 0, lumber: 0, ore: 3 };
        let game  = make_game(b, vec![], white);
        let after = game.apply_move(&GameMove::BuildCity(IntersectionId(10)), Player::White);
        assert!(matches!(after.state.buildings[0].kind, BuildingKind::City));
        assert_eq!(after.state.resources.white.grain, 0);
        assert_eq!(after.state.resources.white.ore,   0);
    }

    // ── generate_moves ────────────────────────────────────────────────────────

    #[test]
    fn test_generate_moves_only_pass_when_no_resources() {
        let game = make_game(vec![], vec![], zero_rc());
        let moves = game.generate_moves(Player::White);
        assert_eq!(moves.len(), 1);
        assert!(matches!(moves[0], GameMove::Pass));
    }

    #[test]
    fn test_generate_moves_includes_build_road_when_affordable() {
        let roads = vec![Road { id: PathId(54), player: Player::White }];
        let white = ResourceCount { grain: 0, wool: 0, brick: 1, lumber: 1, ore: 0 };
        let game  = make_game(vec![], roads, white);
        let moves = game.generate_moves(Player::White);
        assert!(moves.iter().any(|m| matches!(m, GameMove::BuildRoad(_))));
    }

    #[test]
    fn test_generate_moves_includes_build_city_when_settlement_exists() {
        let b = vec![Building { intersection_id: IntersectionId(0), kind: BuildingKind::Settlement, player: Player::White }];
        let white = ResourceCount { grain: 2, wool: 0, brick: 0, lumber: 0, ore: 3 };
        let game  = make_game(b, vec![], white);
        let moves = game.generate_moves(Player::White);
        assert!(moves.iter().any(|m| matches!(m, GameMove::BuildCity(_))));
    }

    #[test]
    fn test_generate_moves_no_city_when_only_city_exists() {
        // An existing City cannot be upgraded again.
        let b = vec![Building { intersection_id: IntersectionId(0), kind: BuildingKind::City, player: Player::White }];
        let white = ResourceCount { grain: 2, wool: 0, brick: 0, lumber: 0, ore: 3 };
        let game  = make_game(b, vec![], white);
        let moves = game.generate_moves(Player::White);
        assert!(!moves.iter().any(|m| matches!(m, GameMove::BuildCity(_))));
    }

    // ── Display ───────────────────────────────────────────────────────────────

    #[test]
    fn test_game_move_display() {
        assert_eq!(GameMove::Pass.to_string(), "pass");
        assert_eq!(GameMove::BuildRoad(Path(IntersectionId(1), IntersectionId(2))).to_string(), "build_road 1 2");
        assert_eq!(GameMove::BuildSettlement(IntersectionId(5)).to_string(), "build_settlement 5");
        assert_eq!(GameMove::BuildCity(IntersectionId(7)).to_string(), "build_city 7");
    }
}
