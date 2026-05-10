# Settlers of Catan AI

A Settlers of Catan AI engine written in Rust, deployable as a WebAssembly service on [Fastly Compute](https://www.fastly.com/products/edge-cloud-platforms/compute).

## How it works

The service accepts a POST request whose body is an ASCII representation of the current game state, runs a **paranoid minimax** search, and returns the best move as a plain-text string.

```
POST /          body: <game state>
200 OK          body: build_settlement 19
                   or build_city 29
                   or build_road 38 39
                   or pass
```

## Algorithm

**Paranoid minimax** with a depth-3 search tree (one full round of White → Red → Blue):

- **Maximiser** (the requesting player): picks the move with the highest heuristic score.
- **Minimisers** (all opponents): assumed to pick moves that minimise the maximiser's score — the "paranoid" worst-case assumption for multi-player games.

**Heuristic score** for a player:

| Component | Formula |
|-----------|---------|
| Victory points | VP × 100 |
| Road progress | road_length × 3 (capped at 200 when ≥ 5, the longest-road bonus) |
| Resource income | Σ dice_probability(tile) × building_multiplier (Settlement=1×, City=2×) |

**Move set** per turn: `BuildRoad`, `BuildSettlement`, `BuildCity`, `Pass`.

## Game state format

The state is encoded as a fixed-width ASCII board plus three resource lines.

```
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
W 1 2 3 4 5
R 6 7 8 9 10
B 11 12 13 14 15
```

### Tiles (`TTTT`)
Four characters: two-digit dice roll + resource letter + optional robber `!`.

| Letter | Resource |
|--------|----------|
| `G` | Grain |
| `W` | Wool |
| `B` | Brick |
| `L` | Lumber |
| `O` | Ore |
| `N` | Nothing (desert) |

### Intersections (`BB`)
Two characters: player letter (`R`/`B`/`W`) + building type (`S`=Settlement, `C`=City), or `oo` for empty.

### Roads (`*`)
One character: player letter (`R`/`B`/`W`) or `.` for empty.

### Resource lines
One line per player: `<player> <grain> <wool> <brick> <lumber> <ore>`

## Module map

| Module | File | Purpose |
|--------|------|---------|
| `main` | `src/main.rs` | Fastly Compute entry point (HTTP handler) |
| `game::board` | `src/game/board.rs` | Core types: `Player`, `Tile`, `Building`, `Road`, `Board`, `State`, `Game` |
| `game::encoding` | `src/game/encoding.rs` | ASCII ↔ `Game` serialization (`TryFrom<String>` / `From<Game>`) |
| `game::resources` | `src/game/resources.rs` | `ResourceCount`, costs, `possible_buys` |
| `moves::possible_moves` | `src/moves/possible_moves.rs` | `possible_road_paths`, `possible_building_intersections`, `longest_road` |
| `moves::maximin` | `src/moves/maximin.rs` | Paranoid minimax + `compute_best_move` |

## Build & run

The binary targets Fastly Compute (WASM). Use `fastly` CLI for local testing:

```bash
# Install Fastly CLI: https://developer.fastly.com/reference/cli/
fastly compute serve --verbose
```

Send a game state:

```bash
curl -X POST http://127.0.0.1:7676 --data-binary @game_state.txt
```

## Tests

```bash
# All unit tests (runs on native target)
cargo test --lib

# Lint
cargo clippy --lib -- -D warnings

# Format check
cargo fmt --all -- --check
```

## Catan build costs (for reference)

| Item | Cost |
|------|------|
| Road | 1 Brick + 1 Lumber |
| Settlement | 1 Grain + 1 Wool + 1 Brick + 1 Lumber |
| City (upgrade) | 2 Grain + 3 Ore |

## License

MIT — see [LICENSE](LICENSE).
