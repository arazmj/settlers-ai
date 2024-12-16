use crate::{IntersectionId, Path};

/// A constant ASCII template representing the Settlers of Catan game board layout.
///
/// The `TEMPLATE` string is a visual representation of the board where:
/// - `BB` represents intersections (potential locations for buildings like settlements or cities).
/// - `*` represents roads connecting intersections.
/// - `TTTT` represents tiles, which hold resources and dice values.
///
/// This template is used as a foundation to overlay dynamic game states, such as player positions,
/// resources, and other game elements, during serialization and rendering of the board.
///
/// Example usage:
/// ```rust
/// println!("{}", TEMPLATE);
/// ```
const TEMPLATE: &str = "
          BB * BB * BB * BB * BB * BB * BB
          *   TTTT  *   TTTT  *   TTTT  *
     BB * BB * BB * BB * BB * BB * BB * BB * BB
     *   TTTT  *   TTTT  *   TTTT  *   TTTT  *
BB * BB * BB * BB * BB * BB * BB * BB * BB * BB * BB
*   TTTT  *   TTTT  *   TTTT  *   TTTT  *   TTTT  *
BB * BB * BB * BB * BB * BB * BB * BB * BB * BB * BB
     *   TTTT  *   TTTT  *   TTTT  *   TTTT  *
     BB * BB * BB * BB * BB * BB * BB * BB * BB
          *   TTTT  *   TTTT  *   TTTT  *
          BB * BB * BB * BB * BB * BB * BB   ";

/// An enumeration representing the players in the Settlers of Catan game.
///
/// Each player is identified by a color:
/// - `Red`: The red player.
/// - `Blue`: The blue player.
/// - `White`: The white player.
///
/// This enum is used to specify the owner of buildings, roads, or other player-specific attributes.
///
/// Example usage:
/// ```rust
/// let current_player = Player::Red;
/// match current_player {
///     Player::Red => println!("Red player's turn"),
///     Player::Blue => println!("Blue player's turn"),
///     Player::White => println!("White player's turn"),
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    Red,
    Blue,
    White,
}

/// Converts a `Player` to its corresponding character representation.
///
/// This implementation maps each player to a unique character:
/// - `Player::Red` -> `'R'`
/// - `Player::Blue` -> `'B'`
/// - `Player::White` -> `'W'`
///
/// Example usage:
/// ```rust
/// let player = Player::Red;
/// let player_char: char = player.into();
/// assert_eq!(player_char, 'R');
/// ```
impl From<crate::Player> for char {
    fn from(player: crate::Player) -> Self {
        match player {
            crate::Player::Red => 'R',
            crate::Player::Blue => 'B',
            crate::Player::White => 'W',
        }
    }
}

/// Attempts to convert a character into a `Player`.
///
/// This implementation maps specific characters to their corresponding `Player` enum:
/// - `'R'` -> `Player::Red`
/// - `'B'` -> `Player::Blue`
/// - `'W'` -> `Player::White`
///
/// Returns an error string if the character is invalid.
///
/// Example usage:
/// ```rust
/// use std::convert::TryFrom;
///
/// let player = Player::try_from('R').unwrap();
/// assert_eq!(player, Player::Red);
///
/// let invalid = Player::try_from('X');
/// assert!(invalid.is_err());
/// ```
impl TryFrom<char> for crate::Player {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(crate::Player::Red),
            'B' => Ok(crate::Player::Blue),
            'W' => Ok(crate::Player::White),
            _ => Err("Invalid character for Player"),
        }
    }
}


/// An enumeration representing the different types of tiles in the Settlers of Catan game.
///
/// Each tile provides a specific resource or has no resource:
/// - `Grain`: Produces grain.
/// - `Wool`: Produces wool.
/// - `Brick`: Produces brick.
/// - `Lumber`: Produces lumber.
/// - `Ore`: Produces ore.
/// - `Nothing`: Represents the desert or other non-productive tiles.
///
/// This enum is used to describe the resource type associated with each tile on the board.
///
/// Example usage:
/// ```rust
/// let tile = TileKind::Grain;
/// match tile {
///     TileKind::Grain => println!("This tile produces grain."),
///     TileKind::Nothing => println!("This is a desert tile."),
///     _ => println!("This tile produces another resource."),
/// }
/// ```
#[derive(Debug, Clone)]
enum  TileKind {
    Grain,
    Wool,
    Brick,
    Lumber,
    Ore,
    Nothing
}


/// Converts a `TileKind` to its corresponding character representation.
///
/// This implementation maps each tile type to a unique character:
/// - `TileKind::Grain` -> `'G'`
/// - `TileKind::Wool` -> `'W'`
/// - `TileKind::Brick` -> `'B'`
/// - `TileKind::Lumber` -> `'L'`
/// - `TileKind::Ore` -> `'O'`
/// - `TileKind::Nothing` -> `'N'`
///
/// Example usage:
/// ```rust
/// let tile = TileKind::Brick;
/// let tile_char: char = tile.into();
/// assert_eq!(tile_char, 'B');
/// ```
impl From<crate::TileKind> for char {
    fn from(tile: crate::TileKind) -> Self {
        match tile {
            crate::TileKind::Grain => 'G',
            crate::TileKind::Wool => 'W',
            crate::TileKind::Brick => 'B',
            crate::TileKind::Lumber => 'L',
            crate::TileKind::Ore => 'O',
            crate::TileKind::Nothing => 'N',
        }
    }
}


/// Attempts to convert a character into a `TileKind`.
///
/// This implementation maps specific characters to their corresponding `TileKind` enum:
/// - `'G'` -> `TileKind::Grain`
/// - `'W'` -> `TileKind::Wool`
/// - `'B'` -> `TileKind::Brick`
/// - `'L'` -> `TileKind::Lumber`
/// - `'O'` -> `TileKind::Ore`
/// - `'N'` -> `TileKind::Nothing`
///
/// Returns an error string if the character does not correspond to a valid `TileKind`.
///
/// # Type Parameters
/// - `Self::Error`: The error type, which is a static string slice (`&'static str`).
///
/// # Arguments
/// - `c`: The character to be converted.
///
/// # Returns
/// A `Result` containing the corresponding `TileKind` if the character is valid, or an error string if invalid.
///
/// # Example
/// ```rust
/// use std::convert::TryFrom;
///
/// let tile_kind = TileKind::try_from('G').unwrap();
/// assert_eq!(tile_kind, TileKind::Grain);
///
/// let invalid_tile = TileKind::try_from('X');
/// assert!(invalid_tile.is_err());
/// ```
impl TryFrom<char> for crate::TileKind {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'G' => Ok(crate::TileKind::Grain),
            'W' => Ok(crate::TileKind::Wool),
            'B' => Ok(crate::TileKind::Brick),
            'L' => Ok(crate::TileKind::Lumber),
            'O' => Ok(crate::TileKind::Ore),
            'N' => Ok(crate::TileKind::Nothing),
            _ => Err("Invalid character for TileKind"),
        }
    }
}


/// A unique identifier for a tile in the Settlers of Catan game.
#[derive(Debug)]
struct TileId(usize);

/// Represents a tile on the game board.
///
/// Each tile has a dice value and a resource type (`TileKind`).


/// Represents a tile on the game board.
///
/// Each tile has a dice value and a resource type (`TileKind`).
#[derive(Debug)]
struct Tile {
    dice: u8,
    kind: crate::TileKind
}

/// An enumeration representing the types of buildings in the Settlers of Catan game.
///
/// - `Settlement`: A basic building that provides fewer points/resources.
/// - `City`: An upgraded building that provides more points/resources.
#[derive(Debug, Copy, Clone)]
enum BuildingKind {
    Settlement,
    City,
}

/// Converts a `BuildingKind` to its corresponding character representation.
///
/// This implementation maps each building type to a unique character:
/// - `BuildingKind::Settlement` -> `'S'`
/// - `BuildingKind::City` -> `'C'`
///
/// Example usage:
/// ```rust
/// let building = BuildingKind::City;
/// assert_eq!(building.to_char(), 'C');
/// ```
impl crate::BuildingKind {
    fn to_char(&self) -> char {
        match self {
            crate::BuildingKind::Settlement => 'S',
            crate::BuildingKind::City => 'C',
        }
    }
}

/// Attempts to convert a character into a `BuildingKind`.
///
/// This implementation maps specific characters to their corresponding `BuildingKind` enum:
/// - `'S'` -> `BuildingKind::Settlement`
/// - `'C'` -> `BuildingKind::City`
///
/// Returns an error string if the character is invalid.
///
/// Example usage:
/// ```rust
/// use std::convert::TryFrom;
///
/// let building = BuildingKind::try_from('S').unwrap();
/// assert_eq!(building, BuildingKind::Settlement);
///
/// let invalid = BuildingKind::try_from('X');
/// assert!(invalid.is_err());
/// ```
impl TryFrom<char> for crate::BuildingKind {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(crate::BuildingKind::Settlement),
            'C' => Ok(crate::BuildingKind::City),
            _ => Err("Invalid character for BuildingKind"),
        }
    }
}


/// Represents a building on the board, including its location (`IntersectionId`),
/// its type (`BuildingKind`), and the player who owns it.
#[derive(Debug, Copy, Clone)]
struct Building {
    intersection_id: IntersectionId,
    kind: crate::BuildingKind,
    player: crate::Player
}


/// Represents a road on the board, including its location (`PathId`)
/// and the player who owns it.
#[derive(Debug)]
struct Road {
    id: crate::PathId,
    player: crate::Player
}


/// A unique identifier for a path (road) in the Settlers of Catan game.
#[derive(Debug, Eq, PartialEq, Hash)]
struct PathId(usize);

#[derive(Debug)]
struct Intersection {
    paths: Vec<crate::PathId>,
    tiles: Vec<crate::TileId>,
}

/// Represents an intersection on the game board.
///
/// Each intersection connects multiple paths (`PathId`) and may touch multiple tiles (`TileId`).
impl crate::Intersection {
    /// Creates a new `Intersection` with the given paths and tiles.
    fn new(paths: Vec<crate::PathId>, tiles: Vec<crate::TileId>) -> crate::Intersection {
        Self {
            paths,
            tiles,
        }
    }
}

const PATHS: usize = 72;
const INTERSECTIONS: usize = 54;
const TILES: usize = 19;

/// Represents the game board in Settlers of Catan.
///
/// The board consists of:
/// - `paths`: An array of roads (`Path`) connecting intersections.
/// - `intersections`: An array of intersections where buildings can be placed.
/// - `tiles`: An array of resource tiles on the board.
struct Board {
    paths: [Path; crate::PATHS],
    intersections: [crate::Intersection; crate::INTERSECTIONS],
    tiles: [crate::Tile; crate::TILES]
}

impl crate::Board {
    /// Creates a new `Board` with the given tiles.
    ///
    /// The paths and intersections are pre-defined for the standard Catan board.
    ///
    /// # Arguments
    /// - `tiles`: An array of 19 tiles representing the game board resources and their dice values.
    ///
    /// Example usage:
    /// ```rust
    /// let tiles = [
    ///     Tile { dice: 10, kind: TileKind::Grain },
    ///     Tile { dice: 2, kind: TileKind::Wool },
    ///     // ...remaining tiles...
    /// ];
    /// let board = Board::new(tiles);
    /// ```
    fn new(tiles: [crate::Tile; 19]) -> crate::Board {
        let paths: [Path;72] = [
            Path(IntersectionId(0), IntersectionId(1)), // 0
            Path(IntersectionId(1), IntersectionId(2)), // 1
            Path(IntersectionId(2), IntersectionId(3)), // 2
            Path(IntersectionId(3), IntersectionId(4)), // 3
            Path(IntersectionId(4), IntersectionId(5)), // 4
            Path(IntersectionId(5), IntersectionId(6)), // 5
            Path(IntersectionId(0), IntersectionId(8)), // 6
            Path(IntersectionId(2), IntersectionId(10)),// 7
            Path(IntersectionId(4), IntersectionId(12)),// 8
            Path(IntersectionId(6), IntersectionId(14)),// 9
            Path(IntersectionId(8), IntersectionId(7)), // 10
            Path(IntersectionId(8), IntersectionId(9)), // 11
            Path(IntersectionId(9), IntersectionId(10)), // 12
            Path(IntersectionId(10), IntersectionId(11)), // 13
            Path(IntersectionId(11), IntersectionId(12)), // 14
            Path(IntersectionId(12), IntersectionId(13)), // 15
            Path(IntersectionId(13), IntersectionId(14)), // 16
            Path(IntersectionId(14), IntersectionId(15)), // 17
            Path(IntersectionId(7), IntersectionId(17)), // 18
            Path(IntersectionId(9), IntersectionId(19)), // 19
            Path(IntersectionId(11), IntersectionId(21)), // 20
            Path(IntersectionId(13), IntersectionId(23)), // 21
            Path(IntersectionId(15), IntersectionId(25)), // 22
            Path(IntersectionId(16), IntersectionId(17)), // 23
            Path(IntersectionId(17), IntersectionId(18)), // 24
            Path(IntersectionId(18), IntersectionId(19)), // 25
            Path(IntersectionId(19), IntersectionId(20)), // 26
            Path(IntersectionId(20), IntersectionId(21)), // 27
            Path(IntersectionId(21), IntersectionId(22)), // 28
            Path(IntersectionId(22), IntersectionId(23)), // 29
            Path(IntersectionId(23), IntersectionId(24)), // 30
            Path(IntersectionId(24), IntersectionId(25)), // 31
            Path(IntersectionId(25), IntersectionId(26)), // 32
            Path(IntersectionId(16), IntersectionId(27)), // 33
            Path(IntersectionId(18), IntersectionId(29)), // 34
            Path(IntersectionId(20), IntersectionId(31)), // 35
            Path(IntersectionId(22), IntersectionId(33)), // 36
            Path(IntersectionId(24), IntersectionId(35)), // 37
            Path(IntersectionId(26), IntersectionId(37)), // 38
            Path(IntersectionId(27), IntersectionId(28)), // 39
            Path(IntersectionId(28), IntersectionId(29)), // 40
            Path(IntersectionId(29), IntersectionId(30)), // 41
            Path(IntersectionId(30), IntersectionId(31)), // 42
            Path(IntersectionId(31), IntersectionId(32)), // 43
            Path(IntersectionId(32), IntersectionId(33)), // 44
            Path(IntersectionId(33), IntersectionId(34)), // 45
            Path(IntersectionId(34), IntersectionId(35)), // 46
            Path(IntersectionId(35), IntersectionId(36)), // 47
            Path(IntersectionId(36), IntersectionId(37)), // 48
            Path(IntersectionId(28), IntersectionId(38)), // 49
            Path(IntersectionId(30), IntersectionId(40)), // 50
            Path(IntersectionId(32), IntersectionId(42)), // 51
            Path(IntersectionId(34), IntersectionId(44)), // 52
            Path(IntersectionId(36), IntersectionId(46)), // 53
            Path(IntersectionId(38), IntersectionId(39)), // 54
            Path(IntersectionId(39), IntersectionId(40)), // 55
            Path(IntersectionId(40), IntersectionId(41)), // 56
            Path(IntersectionId(41), IntersectionId(42)), // 57
            Path(IntersectionId(42), IntersectionId(43)), // 58
            Path(IntersectionId(43), IntersectionId(44)), // 59
            Path(IntersectionId(44), IntersectionId(45)), // 60
            Path(IntersectionId(45), IntersectionId(45)), // 61
            Path(IntersectionId(39), IntersectionId(47)), // 62
            Path(IntersectionId(41), IntersectionId(49)), // 63
            Path(IntersectionId(43), IntersectionId(51)), // 64
            Path(IntersectionId(45), IntersectionId(53)), // 65
            Path(IntersectionId(47), IntersectionId(48)), // 66
            Path(IntersectionId(48), IntersectionId(49)), // 67
            Path(IntersectionId(49), IntersectionId(50)), // 68
            Path(IntersectionId(50), IntersectionId(51)), // 69
            Path(IntersectionId(51), IntersectionId(52)), // 70
            Path(IntersectionId(52), IntersectionId(53)), // 71
        ];

        let intersections: [crate::Intersection; 54] = [
            crate::Intersection::new(vec![crate::PathId(0), crate::PathId(6)], vec![crate::TileId(0)]), // 0
            crate::Intersection::new(vec![crate::PathId(0), crate::PathId(1)], vec![crate::TileId(0)]), // 1
            crate::Intersection::new(vec![crate::PathId(1), crate::PathId(2), crate::PathId(7)], vec![crate::TileId(0), crate::TileId(1)]), // 2
            crate::Intersection::new(vec![crate::PathId(2), crate::PathId(3)], vec![crate::TileId(1)]), // 3
            crate::Intersection::new(vec![crate::PathId(3), crate::PathId(4), crate::PathId(8)], vec![crate::TileId(1), crate::TileId(2)]), // 4
            crate::Intersection::new(vec![crate::PathId(4), crate::PathId(5)], vec![crate::TileId(2)]), // 5
            crate::Intersection::new(vec![crate::PathId(5), crate::PathId(9)], vec![crate::TileId(2)]), // 6
            crate::Intersection::new(vec![crate::PathId(10), crate::PathId(18)], vec![crate::TileId(3)]),  // 7
            crate::Intersection::new(vec![crate::PathId(10), crate::PathId(6), crate::PathId(11)], vec![crate::TileId(3), crate::TileId(0)]),  // 8
            crate::Intersection::new(vec![crate::PathId(11), crate::PathId(12), crate::PathId(19)], vec![crate::TileId(3), crate::TileId(4)]), // 9
            crate::Intersection::new(vec![crate::PathId(7), crate::PathId(12), crate::PathId(13)], vec![crate::TileId(0), crate::TileId(1), crate::TileId(4)]), // 10
            crate::Intersection::new(vec![crate::PathId(13), crate::PathId(14), crate::PathId(20)], vec![crate::TileId(1), crate::TileId(4), crate::TileId(5)]), // 11
            crate::Intersection::new(vec![crate::PathId(8), crate::PathId(14), crate::PathId(15)], vec![crate::TileId(1), crate::TileId(2), crate::TileId(5)]), // 12
            crate::Intersection::new(vec![crate::PathId(15), crate::PathId(16), crate::PathId(21)], vec![crate::TileId(2), crate::TileId(5), crate::TileId(6)]), // 13
            crate::Intersection::new(vec![crate::PathId(9), crate::PathId(16), crate::PathId(17)], vec![crate::TileId(2), crate::TileId(6)]), // 14
            crate::Intersection::new(vec![crate::PathId(17), crate::PathId(22)], vec![crate::TileId(6)]), // 15
            crate::Intersection::new(vec![crate::PathId(23), crate::PathId(33)], vec![crate::TileId(7)]), // 16
            crate::Intersection::new(vec![crate::PathId(18), crate::PathId(23), crate::PathId(24)], vec![crate::TileId(3), crate::TileId(7)]), // 17
            crate::Intersection::new(vec![crate::PathId(24), crate::PathId(25), crate::PathId(34)], vec![crate::TileId(3), crate::TileId(7), crate::TileId(8)]), // 18
            crate::Intersection::new(vec![crate::PathId(19), crate::PathId(25), crate::PathId(26)], vec![crate::TileId(3), crate::TileId(4), crate::TileId(8)]), // 19
            crate::Intersection::new(vec![crate::PathId(26), crate::PathId(27), crate::PathId(35)], vec![crate::TileId(4), crate::TileId(8), crate::TileId(9)]), // 20
            crate::Intersection::new(vec![crate::PathId(20), crate::PathId(27), crate::PathId(28)], vec![crate::TileId(4), crate::TileId(5), crate::TileId(9)]), // 21
            crate::Intersection::new(vec![crate::PathId(28), crate::PathId(29), crate::PathId(36)], vec![crate::TileId(5), crate::TileId(9), crate::TileId(10)]), // 22
            crate::Intersection::new(vec![crate::PathId(21), crate::PathId(29), crate::PathId(30)], vec![crate::TileId(5), crate::TileId(6), crate::TileId(10)]), // 23
            crate::Intersection::new(vec![crate::PathId(30), crate::PathId(31), crate::PathId(37)], vec![crate::TileId(6), crate::TileId(10), crate::TileId(11)]), // 24
            crate::Intersection::new(vec![crate::PathId(22), crate::PathId(31), crate::PathId(32)], vec![crate::TileId(6), crate::TileId(11)]), // 25
            crate::Intersection::new(vec![crate::PathId(32), crate::PathId(38)], vec![crate::TileId(11)]), // 26
            crate::Intersection::new(vec![crate::PathId(33), crate::PathId(39)], vec![crate::TileId(7)]), // 27
            crate::Intersection::new(vec![crate::PathId(39), crate::PathId(40), crate::PathId(49)], vec![crate::TileId(7), crate::TileId(12)]), // 28
            crate::Intersection::new(vec![crate::PathId(34), crate::PathId(40), crate::PathId(41)], vec![crate::TileId(7), crate::TileId(8), crate::TileId(12)]), // 29
            crate::Intersection::new(vec![crate::PathId(41), crate::PathId(42), crate::PathId(50)], vec![crate::TileId(8), crate::TileId(12), crate::TileId(13)]), // 30
            crate::Intersection::new(vec![crate::PathId(35), crate::PathId(42), crate::PathId(43)], vec![crate::TileId(8), crate::TileId(9), crate::TileId(13)]), // 31
            crate::Intersection::new(vec![crate::PathId(43), crate::PathId(44), crate::PathId(51)], vec![crate::TileId(9), crate::TileId(13), crate::TileId(14)]), // 32
            crate::Intersection::new(vec![crate::PathId(36), crate::PathId(44), crate::PathId(45)], vec![crate::TileId(9), crate::TileId(10), crate::TileId(14)]), // 33
            crate::Intersection::new(vec![crate::PathId(45), crate::PathId(46), crate::PathId(52)], vec![crate::TileId(10), crate::TileId(14), crate::TileId(15)]), // 34
            crate::Intersection::new(vec![crate::PathId(37), crate::PathId(46), crate::PathId(47)], vec![crate::TileId(10), crate::TileId(11), crate::TileId(15)]), // 35
            crate::Intersection::new(vec![crate::PathId(47), crate::PathId(48), crate::PathId(53)], vec![crate::TileId(11), crate::TileId(15)]), // 36
            crate::Intersection::new(vec![crate::PathId(38), crate::PathId(48)], vec![crate::TileId(11)]), // 37
            crate::Intersection::new(vec![crate::PathId(49), crate::PathId(54)], vec![crate::TileId(12)]), // 38
            crate::Intersection::new(vec![crate::PathId(54), crate::PathId(55), crate::PathId(62)], vec![crate::TileId(12), crate::TileId(16)]), // 39
            crate::Intersection::new(vec![crate::PathId(50), crate::PathId(55), crate::PathId(56)], vec![crate::TileId(12), crate::TileId(13), crate::TileId(16)]), // 40
            crate::Intersection::new(vec![crate::PathId(56), crate::PathId(57), crate::PathId(63)], vec![crate::TileId(13), crate::TileId(16), crate::TileId(17)]), // 41
            crate::Intersection::new(vec![crate::PathId(51), crate::PathId(57), crate::PathId(58)], vec![crate::TileId(13), crate::TileId(14), crate::TileId(17)]), // 42
            crate::Intersection::new(vec![crate::PathId(58), crate::PathId(59), crate::PathId(64)], vec![crate::TileId(14), crate::TileId(17), crate::TileId(18)]), // 43
            crate::Intersection::new(vec![crate::PathId(52), crate::PathId(59), crate::PathId(60)], vec![crate::TileId(14), crate::TileId(15), crate::TileId(18)]), // 44
            crate::Intersection::new(vec![crate::PathId(60), crate::PathId(61), crate::PathId(65)], vec![crate::TileId(15), crate::TileId(18)]), // 45
            crate::Intersection::new(vec![crate::PathId(53), crate::PathId(61)], vec![crate::TileId(15)]), // 46
            crate::Intersection::new(vec![crate::PathId(62), crate::PathId(66)], vec![crate::TileId(16)]), // 47
            crate::Intersection::new(vec![crate::PathId(66), crate::PathId(67)], vec![crate::TileId(16)]), // 48
            crate::Intersection::new(vec![crate::PathId(63), crate::PathId(67), crate::PathId(68)], vec![crate::TileId(16), crate::TileId(17)]), // 49
            crate::Intersection::new(vec![crate::PathId(68), crate::PathId(69)], vec![crate::TileId(17)]), // 50
            crate::Intersection::new(vec![crate::PathId(64), crate::PathId(69), crate::PathId(70)], vec![crate::TileId(17), crate::TileId(18)]), // 51
            crate::Intersection::new(vec![crate::PathId(70), crate::PathId(71)], vec![crate::TileId(18)]), // 52
            crate::Intersection::new(vec![crate::PathId(65), crate::PathId(71)], vec![crate::TileId(18)]), // 53
        ];

        Self {
            paths,
            intersections,
            tiles,
        }
    }
}

/// A unique identifier for the position of the robber on the game board.
#[derive(Eq, PartialEq)]
struct RobberId(usize);

/// Represents the state of the game, including:
/// - `buildings`: A list of all buildings on the board.
/// - `roads`: A list of all roads on the board.
/// - `robber`: The current position of the robber.
struct State {
    buildings: Vec<crate::Building>,
    roads: Vec<crate::Road>,
    robber: crate::RobberId,
}

/// Represents the overall game state, including the board and the state of all players.
struct Game {
    board: crate::Board,
    state: crate::State,
}