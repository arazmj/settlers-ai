# Settlers of Catan AI

A Settlers of Catan AI engine written in Rust, deployed as a **Fastly Compute** (WebAssembly) service. It accepts an ASCII-encoded game state over HTTP and returns the best move computed by a paranoid minimax search.

## Quick start

```bash
# Requires Fastly CLI: https://developer.fastly.com/reference/cli/
fastly compute serve --verbose

# Send a game state and get the best move for White
curl -s -X POST http://127.0.0.1:7676 --data-binary @game_state.txt
# → build_settlement 19
```

## HTTP API

```
POST /
Content-Type: text/plain

<ASCII game state>
```

**Response** — one of:

| Response | Meaning |
|----------|---------|
| `build_road <a> <b>` | Build a road between intersections *a* and *b* |
| `build_settlement <id>` | Place a settlement at intersection *id* |
| `build_city <id>` | Upgrade the settlement at intersection *id* to a city |
| `pass` | No affordable or legal move |

## Algorithm

**Paranoid minimax**, depth 3 (one full White → Red → Blue round):

- **Maximiser** (the requesting player): picks the move with the highest heuristic score.
- **Minimisers** (all opponents): assumed to minimise the maximiser's score — the standard worst-case assumption for multi-player games.

**Heuristic score:**

| Component | Weight |
|-----------|--------|
| Victory points | VP × 100 |
| Road progress toward longest-road bonus | road\_len × 3; capped at 200 once ≥ 5 |
| Resource income | Σ dice\_prob(tile) × multiplier (Settlement = 1×, City = 2×) |

Dice probability is the number of ways to roll the tile's number on 2d6 (e.g. 6 or 8 → 5, 7 → 0 because of the robber).

**Build costs:**

| Action | Cost |
|--------|------|
| Road | 1 Brick + 1 Lumber |
| Settlement | 1 Grain + 1 Wool + 1 Brick + 1 Lumber |
| City (upgrade) | 2 Grain + 3 Ore |

## Game state format

A fixed-width 11-line ASCII board followed by three resource lines.

```
          oo . oo . oo . oo . oo W oo W oo     ← row 0
          .   10O   .   02W   .   09L   W       ← row 1
     oo . oo . oo . RS R oo . oo B BS W oo . oo
     .   12G   .   06B   .   04W   W   10B   .
oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   W   08O   .
oo . oo . RS R oo . oo . oo . oo . oo . WS . oo . oo
     .   08L   .   03O   .   04G   B   05W   .
     oo . oo . RS B oo . oo . oo . RS . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo     ← row 10
W 1 2 3 4 5       ← White: grain wool brick lumber ore
R 6 7 8 9 10
B 11 12 13 14 15
```

**Board topology:** 54 intersections · 72 paths · 19 tiles (standard Catan layout).

### Tiles — 4 chars `DDKX`

| Position | Meaning |
|----------|---------|
| `DD` | Two-digit dice roll (`00`–`12`) |
| `K` | Resource: `G`rain `W`ool `B`rick `L`umber `O`re `N`othing |
| `X` | `!` if the robber is here, space otherwise |

### Intersections — 2 chars `PK`

| Value | Meaning |
|-------|---------|
| `oo` | Empty |
| `RS` `RC` | Red Settlement / City |
| `BS` `BC` | Blue Settlement / City |
| `WS` `WC` | White Settlement / City |

### Roads — 1 char

| Value | Meaning |
|-------|---------|
| `.` | Empty path |
| `R` `B` `W` | Road owned by Red / Blue / White |

## Module map

| Module | File | Purpose |
|--------|------|---------|
| `main` | `src/main.rs` | Fastly Compute entry point; routes POST to `compute_best_move` |
| `game::board` | `src/game/board.rs` | Core types: `Player`, `Tile`, `Building`, `Road`, `Board`, `State`, `Game` |
| `game::encoding` | `src/game/encoding.rs` | ASCII ↔ `Game` round-trip (`TryFrom<String>` / `From<Game>`) |
| `game::resources` | `src/game/resources.rs` | `ResourceCount`, build costs, `possible_buys` |
| `moves::possible_moves` | `src/moves/possible_moves.rs` | `possible_road_paths`, `possible_building_intersections`, `longest_road` |
| `moves::maximin` | `src/moves/maximin.rs` | `GameMove` enum, `apply_move`, minimax tree, `compute_best_move` |

### Data flow

```
HTTP POST body
  └─ encoding::TryFrom<String> → Game
       └─ maximin::compute_best_move(player)
            ├─ generate_moves → [BuildRoad, BuildSettlement, BuildCity, Pass]
            ├─ apply_move (clone state, deduct resources, place piece)
            └─ minimax(depth=3) → best GameMove → Display → HTTP response
```

## Build

The binary targets `wasm32-wasi` (required by Fastly Compute):

```bash
# Build WASM package (done automatically by `fastly compute serve`)
cargo build --bin settlers --release --target wasm32-wasi

# Library tests run on the native target (no WASM toolchain needed)
cargo test --lib
cargo clippy --lib -- -D warnings
cargo fmt --all -- --check
```

The test suite currently has **50 tests** covering encoding round-trips, resource arithmetic, longest-road calculation, possible-move generation, minimax scoring, move application, and move selection.

## License

MIT — see [LICENSE](LICENSE).
