use crate::game::{Building, IntersectionId, Path, Player};
use std::collections::{HashMap, HashSet};
use crate::game::Game;

// Backtracking DFS that tracks visited edges (not nodes), so cyclic road networks
// are handled correctly. Catan roads can form loops; a node may be revisited as
// long as the edge used to reach it hasn't been traversed yet in this path.
#[allow(dead_code)]
fn dfs_edges(
    node: usize,
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    visited_edges: &mut HashSet<usize>,
    depth: usize,
) -> usize {
    let mut max_depth = depth;
    if let Some(neighbors) = graph.get(&node) {
        for &(next, edge_id) in neighbors {
            if !visited_edges.contains(&edge_id) {
                visited_edges.insert(edge_id);
                let result = dfs_edges(next, graph, visited_edges, depth + 1);
                max_depth = max_depth.max(result);
                visited_edges.remove(&edge_id);
            }
        }
    }
    max_depth
}

impl Game {
    #[allow(dead_code)]
    pub(crate) fn longest_road(&self, player: Player) -> usize {
        let graph = self.road_graph_with_edges(player);
        graph.keys()
            .map(|&start| dfs_edges(start, &graph, &mut HashSet::new(), 0))
            .max()
            .unwrap_or(0)
    }

    #[allow(dead_code)]
    fn road_graph_with_edges(&self, player: Player) -> HashMap<usize, Vec<(usize, usize)>> {
        let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        for road in &self.state.roads {
            if road.player == player {
                let Path(IntersectionId(a), IntersectionId(b)) = self.board.paths[road.id.0];
                let edge_id = road.id.0;
                graph.entry(a).or_default().push((b, edge_id));
                graph.entry(b).or_default().push((a, edge_id));
            }
        }
        graph
    }

    /// Constructs a graph of roads owned by the given player.
    ///
    /// The graph is represented as a mapping of intersection IDs to lists of connected intersections.
    ///
    /// # Arguments
    /// - `player`: The player whose roads are being considered.
    ///
    /// # Returns
    /// A `HashMap` representing the graph of roads owned by the player.
    fn road_graph(&self, player: Player) -> HashMap<usize, Vec<usize>> {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for road in &self.state.roads {
            if road.player == player {
                let Path(IntersectionId(a), IntersectionId(b)) = self.board.paths[road.id.0];
                graph.entry(a).or_default().push(b);
                graph.entry(b).or_default().push(a);
            }
        }
        graph
    }

    /// Identifies intersections where the given player can build a new building.
    ///
    /// Ensures that buildings are not placed too close to each other.
    ///
    /// # Arguments
    /// - `player`: The player attempting to build.
    ///
    /// # Returns
    /// A `HashSet` of `IntersectionId`s where the player can build.
    pub(crate) fn possible_building_intersections(&self, player: Player) -> HashSet<IntersectionId> {
        let too_close_intersections= self.too_close_intersections();
        let mut possible_building_intersections: HashSet<IntersectionId> = HashSet::new();
        for road in self.state.roads.iter().filter(|road| road.player == player) {
            let Path(intersection_a, intersection_b) =  self.board.paths[road.id.0];
            if !too_close_intersections.contains(&intersection_a) {
                possible_building_intersections.insert(intersection_a);
            }
            if !too_close_intersections.contains(&intersection_b) {
                possible_building_intersections.insert(intersection_b);
            }
        }
        possible_building_intersections
    }


    /// Determines all possible road paths a player can build on the board.
    ///
    /// This function identifies potential road locations based on the current state of the board and the roads
    /// already owned by the player. It ensures that new roads are connected to existing roads owned by the player
    /// and do not conflict with existing roads owned by other players.
    ///
    /// # Arguments
    /// - `player`: The `Player` whose potential road paths are being evaluated.
    ///
    /// # Returns
    /// A `HashSet` containing all possible `Path`s where the player can build new roads.
    ///
    /// # Methodology
    /// 1. Builds a road graph of intersections connected by roads owned by the player.
    /// 2. Identifies "leaf" intersections—intersections with exactly one connected road—where new roads can branch out.
    /// 3. Collects all valid paths where a new road can connect to these leaf intersections.
    ///
    /// # Example
    /// ```no_run
    /// let possible_paths = game.possible_road_paths(Player::Red);
    /// for path in possible_paths {
    ///     println!("Possible path: {:?} -> {:?}", path.0, path.1);
    /// }
    /// ```
    pub(crate) fn possible_road_paths(&self, player: Player) -> HashSet<Path> {
        let graph = self.road_graph(player);
        // Any intersection connected to the player's road network is a valid origin.
        // (Leaf-only detection was wrong for cyclic road networks: a ring has no
        // degree-1 nodes, so the old code returned an empty set.)
        let mut player_intersections: HashSet<usize> = graph.keys().copied().collect();
        // Roads can also be anchored at the player's own settlements / cities.
        for b in self.state.buildings.iter().filter(|b| b.player == player) {
            player_intersections.insert(b.intersection_id.0);
        }
        let occupied_path_ids: HashSet<usize> = self.state.roads.iter()
            .map(|r| r.id.0)
            .collect();

        let mut possible_road_paths: HashSet<Path> = HashSet::new();
        for (path_id, path) in self.board.paths.iter().enumerate() {
            if occupied_path_ids.contains(&path_id) {
                continue;
            }
            let Path(IntersectionId(a), IntersectionId(b)) = path;
            if player_intersections.contains(a) || player_intersections.contains(b) {
                possible_road_paths.insert(path.clone());
            }
        }
        possible_road_paths
    }

    /// Constructs a set of intersections that are too close to existing buildings.
    ///
    /// Buildings cannot be placed adjacent to other buildings.
    ///
    /// # Returns
    /// A `HashSet` of `IntersectionId`s that are too close to existing buildings.
    pub(crate) fn too_close_intersections(&self) -> HashSet<IntersectionId> {
        let mut too_close_intersections: HashSet<IntersectionId> = HashSet::new();
        for building in self.state.buildings.iter() {
            let Building{
                intersection_id,
                ..
            } = building;

            for path in  &self.board.intersections[intersection_id.0].paths {
                let Path(intersection_a, intersection_b) = self.board.paths[path.0];
                too_close_intersections.insert(intersection_a);
                too_close_intersections.insert(intersection_b);

            }
        }
        too_close_intersections
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::convert::TryInto;
    // Bug: path 61 was Path(45, 45) — a self-loop that made intersection 46 unreachable.
    #[test]
    fn test_path_61_not_self_loop() {
        let game: Game = "
          oo . oo . oo . oo . oo . oo . oo
          .   10O   .   02W   .   09L   .
     oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   12G   .   06B   .   04W   .   10B   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   .   08O   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   08L   .   03O   .   04G   .   05W   .
     oo . oo . oo . oo . oo . oo . oo . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 0 0 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        let Path(IntersectionId(a), IntersectionId(b)) = game.board.paths[61];
        assert_ne!(a, b, "path 61 was a self-loop (45→45); must connect distinct intersections");
        assert_eq!((a, b), (45, 46));
    }

    // Bug: longest_road always started DFS from intersection 6, panicking with
    // "key not found" when the player had no roads touching that intersection.
    #[test]
    fn test_longest_road_roads_not_at_intersection_6() {
        // Red roads are paths 54(38-39), 55(39-40), 56(40-41) — bottom row only.
        let game: Game = "
          oo . oo . oo . oo . oo . oo . oo
          .   10O   .   02W   .   09L   .
     oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   12G   .   06B   .   04W   .   10B   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   .   08O   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   08L   .   03O   .   04G   .   05W   .
     oo R oo R oo R oo . oo . oo . oo . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 0 0 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        assert_eq!(game.longest_road(Player::Red), 3);
    }

    // Bug: dfs() discarded the `longest` returned by recursive calls, so the true
    // diameter was lost when it passed through a node that was not the DFS root.
    //
    // Red road network (paths 9,16,17,21,22,30,32):
    //
    //   6 ─ 14 ─ 13 ─ 23 ─ 24
    //         └─ 15 ─ 25 ─ 26
    //
    // Longest road: 24-23-13-14-15-25-26 = 6 edges (diameter through node 14).
    // Old code started DFS at node 6 and discarded the 6 found at node 14, returning 4.
    #[test]
    fn test_longest_road_diameter_through_internal_node() {
        let game: Game = "
          oo . oo . oo . oo . oo . oo . oo
          .   10O   .   02W   .   09L   R
     oo . oo . oo . oo . oo . oo . oo R oo R oo
     .   12G   .   06B   .   04W   R   10B   R
oo . oo . oo . oo . oo . oo . oo . oo R oo . oo R oo
.   09G!  .   11L   .   00N   .   03L   .   08O   .
oo . oo . oo . oo . oo . oo . oo . oo . oo . oo . oo
     .   08L   .   03O   .   04G   .   05W   .
     oo . oo . oo . oo . oo . oo . oo . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 0 0 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        assert_eq!(game.longest_road(Player::Red), 6);
    }

    #[test]
    fn test_possible_possible_road_paths() {
        let game: Game = "
          oo . oo . oo . oo . oo W oo W oo
          .   10O   .   02W   .   09L   W
     oo . oo . oo . RS R oo . oo B BS W oo . oo
     .   12G   .   06B   .   04W   W   10B   .
oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   W   08O   .
oo . oo . RS R oo . oo . oo . oo . oo . WS W oo . oo
     .   08L   .   03O   .   04G   B   05W   W
     oo . oo . RS B oo . oo . oo . RS . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 0 0 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        // All unoccupied paths adjacent to any White road intersection (not just
        // degree-1 leaf nodes — interior junctions are valid extension points too).
        let s: HashSet<Path> = vec![
            Path(IntersectionId(3),  IntersectionId(4)),
            Path(IntersectionId(4),  IntersectionId(12)),
            Path(IntersectionId(9),  IntersectionId(19)),
            Path(IntersectionId(14), IntersectionId(15)),
            Path(IntersectionId(17), IntersectionId(18)),
            Path(IntersectionId(18), IntersectionId(29)),
            Path(IntersectionId(19), IntersectionId(20)),
            Path(IntersectionId(22), IntersectionId(23)),
            Path(IntersectionId(24), IntersectionId(25)),
            Path(IntersectionId(34), IntersectionId(35)),
            Path(IntersectionId(36), IntersectionId(37)),
            Path(IntersectionId(45), IntersectionId(46)),
        ].into_iter().collect();
        assert_eq!(s, game.possible_road_paths(Player::White));
    
    }



    #[test]
    fn test_longest_road() {
        let game: Game = "
          oo . oo . oo . oo . oo W oo W oo
          .   10O   .   02W   .   09L   W
     oo . oo . oo . RS R oo . oo B BS W oo . oo
     .   12G   .   06B   .   04W   W   10B   .
oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   W   08O   .
oo . oo . RS R oo . oo . oo . oo . oo . WS . oo . oo
     .   08L   .   03O   .   04G   B   05W   .
     oo . oo . RS B oo . oo . oo . RS . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 0 0 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        assert_eq!(game.longest_road(Player::White), 7);
    }

    #[test]
    fn test_possible_building_intersections() {
        let game: Game = "
          oo . oo . oo . oo . oo W oo W oo
          .   10O   .   02W   .   09L   W
     oo . oo . oo . RS R oo . oo B BS W oo . oo
     .   12G   .   06B   .   04W   W   10B   .
oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   W   08O   .
oo . oo . RS R oo . oo . oo . oo . oo . WS W oo . oo
     .   08L   .   03O   .   04G   B   05W   W
     oo . oo . RS B oo . oo . oo . RS . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo
W 0 0 0 0 0
R 0 0 0 0 0
B 0 0 0 0 0".to_string().try_into().unwrap();
        let s: HashSet<IntersectionId> = vec![IntersectionId(46), IntersectionId(6), IntersectionId(4), IntersectionId(5)].into_iter().collect();
        assert_eq!(game.possible_building_intersections(Player::White), s);
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    fn make_game(buildings: Vec<Building>, roads: Vec<crate::game::Road>) -> Game {
        use crate::game::{Board, RobberId, Tile, TileKind};
        use crate::game::resources::{PlayerResourceCount, ResourceCount};
        let zero = ResourceCount { grain: 0, wool: 0, brick: 0, lumber: 0, ore: 0 };
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
        Game {
            board: Board::new(tiles),
            state: crate::game::State {
                buildings,
                roads,
                robber: RobberId(9), // desert tile
                resources: PlayerResourceCount { red: zero.clone(), blue: zero.clone(), white: zero },
            },
        }
    }

    // ── new tests ─────────────────────────────────────────────────────────────

    #[test]
    fn test_longest_road_no_roads() {
        let game = make_game(vec![], vec![]);
        assert_eq!(game.longest_road(Player::White), 0);
    }

    #[test]
    fn test_longest_road_single_road() {
        // Path 54 connects intersections 38–39.
        let roads = vec![crate::game::Road { id: crate::game::PathId(54), player: Player::White }];
        let game = make_game(vec![], roads);
        assert_eq!(game.longest_road(Player::White), 1);
    }

    #[test]
    fn test_possible_road_paths_empty_when_no_roads() {
        // With no existing roads, there are no endpoints to extend from.
        let game = make_game(vec![], vec![]);
        assert!(game.possible_road_paths(Player::White).is_empty());
    }

    #[test]
    fn test_too_close_intersections_empty_board() {
        let game = make_game(vec![], vec![]);
        assert!(game.too_close_intersections().is_empty());
    }

    #[test]
    fn test_too_close_intersections_one_building() {
        // Intersection 10 connects via paths 7 (2–10), 12 (9–10), 13 (10–11).
        // So building at 10 should block intersections {2, 9, 10, 11}.
        let buildings = vec![Building {
            intersection_id: IntersectionId(10),
            kind: crate::game::BuildingKind::Settlement,
            player: Player::White,
        }];
        let game = make_game(buildings, vec![]);
        let blocked = game.too_close_intersections();
        for id in [2, 9, 10, 11] {
            assert!(blocked.contains(&IntersectionId(id)), "expected {} to be blocked", id);
        }
        // Only the 4 intersections touched by paths of intersection 10.
        assert_eq!(blocked.len(), 4);
    }

    #[test]
    fn test_possible_road_paths_from_settlement_no_existing_road() {
        // Intersection 38 sits on paths 49 (28–38) and 54 (38–39).
        // A settlement there should anchor roads even without any existing roads.
        let buildings = vec![Building {
            intersection_id: IntersectionId(38),
            kind: crate::game::BuildingKind::Settlement,
            player: Player::White,
        }];
        let game = make_game(buildings, vec![]);
        let paths = game.possible_road_paths(Player::White);
        assert!(!paths.is_empty(), "settlement should enable road placement");
        assert!(paths.contains(&Path(IntersectionId(28), IntersectionId(38))), "path 49 (28–38) should be reachable");
        assert!(paths.contains(&Path(IntersectionId(38), IntersectionId(39))), "path 54 (38–39) should be reachable");
        assert_eq!(paths.len(), 2, "only 2 paths touch intersection 38");
    }
}