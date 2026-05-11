use std::collections::HashSet;
use std::ops::{Add, Index, Sub};
use crate::game::{Player, TileKind};

#[derive(Debug, PartialEq, Clone)]
pub struct ResourceCount {
    pub grain: i8,
    pub wool: i8,
    pub brick: i8,
    pub lumber: i8,
    pub ore: i8,
}

#[derive(Eq, Hash, PartialEq, Debug)]
#[allow(dead_code)]
enum Buys {
    Road,
    Settlement,
    City
}

impl Index<TileKind> for ResourceCount {
    type Output = i8;
    fn index(&self, tile: TileKind) -> &Self::Output {
        match tile {
            TileKind::Grain => &self.grain,
            TileKind::Wool => &self.wool,
            TileKind::Brick => &self.brick,
            TileKind::Lumber => &self.lumber,
            TileKind::Ore => &self.ore,
            TileKind::Nothing => &0
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct PlayerResourceCount {
    pub red: ResourceCount,
    pub blue: ResourceCount,
    pub white: ResourceCount,
}

impl Index<Player> for PlayerResourceCount {
    type Output = ResourceCount;

    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::Red => &self.red,
            Player::Blue => &self.blue,
            Player::White => &self.white
        }
    }
}


#[allow(dead_code)]
pub(crate) const ROAD_COST: ResourceCount = ResourceCount{
    grain: 0,
    wool: 0,
    brick: 1,
    lumber: 1,
    ore: 0,
};

#[allow(dead_code)]
pub(crate) const SETTLEMENT_COST: ResourceCount = ResourceCount{
    grain: 1,
    wool: 1,
    brick: 1,
    lumber: 1,
    ore: 0,
};

#[allow(dead_code)]
pub(crate) const CITY_COST: ResourceCount = ResourceCount{
    grain: 2,
    wool: 0,
    brick: 0,
    lumber: 0,
    ore: 3,
};

#[allow(dead_code)]
fn possible_buys_dfs(resource_count: ResourceCount, buys: &mut HashSet<Buys>) {
    let zip = [ROAD_COST, SETTLEMENT_COST, CITY_COST].iter().zip([Buys::Road, Buys::Settlement, Buys::City]);
    for (cost, buy) in zip {
        let sub_count = resource_count.clone() - cost.clone();
        if sub_count.is_positive() {
            buys.insert(buy);
            possible_buys_dfs(sub_count, buys);
        }
    }
}

impl ResourceCount {
    pub fn is_positive(&self) -> bool {
        self.grain >= 0 && self.wool >= 0 && self.brick >= 0 && self.lumber >= 0 && self.ore >= 0
    }

    #[allow(dead_code)]
    fn possible_buys(&self) -> HashSet<Buys> {
        let mut buys: HashSet<Buys> = HashSet::new();
        possible_buys_dfs(self.clone(), &mut buys);
        buys
    }

}

impl Sub<Self> for ResourceCount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            grain: self.grain - rhs.grain,
            wool: self.wool - rhs.wool,
            brick: self.brick - rhs.brick,
            lumber: self.lumber - rhs.lumber,
            ore: self.ore - rhs.ore,
        }
    }
}

impl Add<Self> for ResourceCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            grain: self.grain + rhs.grain,
            wool: self.wool + rhs.wool,
            brick: self.brick + rhs.brick,
            lumber: self.lumber + rhs.lumber,
            ore: self.ore + rhs.ore,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::convert::TryInto;
    use crate::game::Game;
    use crate::game::resources::{Buys, PlayerResourceCount, ResourceCount, SETTLEMENT_COST};

    #[test]
    fn test_parse_resources() {
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
   G  W  B  L  O
W  1  2  3  4  5  
R  6  7  8  9  10 
B  11 12 13 14 15"
            .to_string()
            .try_into()
            .unwrap();
        let s = PlayerResourceCount {
            red: ResourceCount {
                grain: 6,
                wool: 7,
                brick: 8,
                lumber: 9,
                ore: 10,
            },
            blue: ResourceCount {
                grain: 11,
                wool: 12,
                brick: 13,
                lumber: 14,
                ore: 15,
            },
            white: ResourceCount {
                grain: 1,
                wool: 2,
                brick: 3,
                lumber: 4,
                ore: 5,
            },
        };
        assert_eq!(s, game.state.resources);
    }


    #[test]
    fn test_calculate_resources() {
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
   G  W  B  L  O
W  1  1  1  1  1
R  6  7  8  9  10
B  11 12 13 14 15"
            .to_string()
            .try_into()
            .unwrap();

        let s = ResourceCount { grain: 0, wool: 0, brick: 0, lumber: 0, ore: 1 };
        let x = game.state.resources.white - SETTLEMENT_COST;
        assert_eq!(s, x);
    }

    #[test]
    fn test_possible_buys() {
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
   G  W  B  L  O
W  1  1  1  1  1
R  6  7  8  9  10
B  11 12 13 14 15"
            .to_string()
            .try_into()
            .unwrap();

        let buys = game.state.resources.white.possible_buys();

        let a: HashSet<Buys>  = vec![Buys::Road, Buys::Settlement].into_iter().collect();

        assert_eq!(a, buys);
    }

    #[test]
    fn test_resource_count_add() {
        let a = ResourceCount { grain: 1, wool: 2, brick: 3, lumber: 4, ore: 5 };
        let b = ResourceCount { grain: 5, wool: 4, brick: 3, lumber: 2, ore: 1 };
        assert_eq!(a + b, ResourceCount { grain: 6, wool: 6, brick: 6, lumber: 6, ore: 6 });
    }

    #[test]
    fn test_resource_count_sub() {
        let a = ResourceCount { grain: 3, wool: 3, brick: 3, lumber: 3, ore: 3 };
        let b = ResourceCount { grain: 1, wool: 1, brick: 1, lumber: 1, ore: 1 };
        assert_eq!(a - b, ResourceCount { grain: 2, wool: 2, brick: 2, lumber: 2, ore: 2 });
    }

    #[test]
    fn test_is_positive_all_zero() {
        // Zero counts are non-negative, so is_positive should return true.
        let r = ResourceCount { grain: 0, wool: 0, brick: 0, lumber: 0, ore: 0 };
        assert!(r.is_positive());
    }

    #[test]
    fn test_is_not_positive_when_any_negative() {
        let r = ResourceCount { grain: 1, wool: 1, brick: -1, lumber: 1, ore: 1 };
        assert!(!r.is_positive());
    }

    #[test]
    fn test_possible_buys_empty_when_no_resources() {
        let r = ResourceCount { grain: 0, wool: 0, brick: 0, lumber: 0, ore: 0 };
        assert!(r.possible_buys().is_empty());
    }

    #[test]
    fn test_possible_buys_road_only() {
        // brick=1 + lumber=1 → exactly one road affordable, nothing else.
        let r = ResourceCount { grain: 0, wool: 0, brick: 1, lumber: 1, ore: 0 };
        let buys = r.possible_buys();
        assert_eq!(buys, vec![Buys::Road].into_iter().collect::<HashSet<_>>());
    }

    #[test]
    fn test_possible_buys_includes_city_when_affordable() {
        // 2 grain + 3 ore → City must be in the set.
        let r = ResourceCount { grain: 2, wool: 0, brick: 0, lumber: 0, ore: 3 };
        let buys = r.possible_buys();
        assert!(buys.contains(&Buys::City));
    }
}

