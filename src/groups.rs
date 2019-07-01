use crate::tiles::*;
use crate::groups::Set::{Chow, Pung, Pair};

pub use std::str::FromStr;
use std::fmt::{Display, Formatter, Error, Debug};
use crate::score::Fu;

/// 複数枚の牌に関する情報
pub trait Tiles {
    /// 么九牌を含むかどうか
    fn contains_yaotyu(&self) -> bool;
    /// 么九牌のみかどうか
    fn all_yaotyu(&self) -> bool;
    /// 一九牌を含むかどうか
    fn contains_terminal(&self) -> bool;
    /// 一九牌のみかどうか
    fn all_terminal(&self) -> bool;
    /// 連続した並び(1,2,3や6,7,8,9など)かどうか
    fn is_sequential(&self) -> bool;
    /// 刻子のような並び(1,1,1や6,6,6,6など)かどうか
    fn is_flat(&self) -> bool;
    /// 該当する牌の集計
    fn count(&self, tile: &Tile) -> u8;
    /// 三色同順,三食同刻の判定に利用する
    fn sum_tile(&self) -> Option<Tile>;
}

impl Tiles for Vec<Tile> {
    fn contains_yaotyu(&self) -> bool {
        self.iter().any(|tile| tile.is_yaotyu())
    }

    fn all_yaotyu(&self) -> bool {
        self.iter().all(|tile| tile.is_yaotyu())
    }

    fn contains_terminal(&self) -> bool {
        self.iter().any(|tile| tile.is_terminal())
    }

    fn all_terminal(&self) -> bool {
        self.iter().all(|tile| tile.is_terminal())
    }

    fn is_sequential(&self) -> bool {
        let mut iter = self.iter().peekable();
        while let (Some(n), Some(p)) = (iter.next(), iter.peek()) {
            // validation
            match n {
                Tile::Character(u) => {
                    if p != &&Tile::Character(u + 1) {
                        return false;
                    }
                }
                Tile::Circle(u) => {
                    if p != &&Tile::Circle(u + 1) {
                        return false;
                    }
                }
                Tile::Bamboo(u) => {
                    if p != &&Tile::Bamboo(u + 1) {
                        return false;
                    }
                }
                _ => {
                    // 字牌は並びを持たない
                    return false;
                }
            }
        }
        return true;
    }

    fn is_flat(&self) -> bool {
        let mut iter = self.iter();
        match iter.next() {
            Some(tile) => {
                iter.all(|t| tile == t)
            }
            None => {
                false
            }
        }
    }

    fn count(&self, tile: &Tile) -> u8 {
        let mut count: u8 = 0;
        for t in self {
            if t == tile { count += 1; }
        }
        count
    }

    fn sum_tile(&self) -> Option<Tile> {
        let mut sum = 0;
        self.iter().for_each(|tile| {
            sum +=
                if tile.is_honours() {
                    // 字牌は0
                    0
                } else {
                    match tile {
                        Tile::Character(u) => u,
                        Tile::Circle(u) => u,
                        Tile::Bamboo(u) => u,
                        _ => unreachable!()
                    }.clone()
                }
        });
        match self.get(0) {
            Some(tile) => {
                match tile {
                    Tile::Character(_) => Some(Tile::Character(sum)),
                    Tile::Circle(_) => Some(Tile::Circle(sum)),
                    Tile::Bamboo(_) => Some(Tile::Bamboo(sum)),
                    _ => None,
                }
            }
            None => None
        }
    }
}

pub struct TilesNewType(pub Vec<Tile>);

impl FromStr for TilesNewType {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::with_capacity(14);

        // パース
        let mut iter = s.chars().peekable();

        while let Some(c) = iter.next() {
            let tile =
                match c.to_string().parse::<u8>() {
                    Ok(u) => {
                        if u == 0 {
                            return Err(format_err!("数字が不正です: {} in {}",u,s));
                        }
                        match iter.next() {
                            Some(c) => {
                                if Tile::characters_markers().contains(&c) {
                                    // 萬子
                                    Tile::Character(u)
                                } else if Tile::circles_markers().contains(&c) {
                                    // 筒子
                                    Tile::Circle(u)
                                } else if Tile::bamboos_markers().contains(&c) {
                                    // 索子
                                    Tile::Bamboo(u)
                                } else {
                                    match c.to_string().parse::<u8>() {
                                        // 数字
                                        Ok(u2) => {
                                            // 123m456p789sのような省略記法の場合
                                            let mut nums = Vec::with_capacity(14);
                                            nums.push(u);
                                            nums.push(u2);
                                            while let Some(c) = iter.next() {
                                                let mut tiles_tmp: Vec<Tile> =
                                                    if Tile::characters_markers().contains(&c) {
                                                        // 萬子
                                                        nums.iter().map(|u| Tile::Character(u.clone())).collect()
                                                    } else if Tile::circles_markers().contains(&c) {
                                                        // 筒子
                                                        nums.iter().map(|u| Tile::Circle(u.clone())).collect()
                                                    } else if Tile::bamboos_markers().contains(&c) {
                                                        // 索子
                                                        nums.iter().map(|u| Tile::Bamboo(u.clone())).collect()
                                                    } else {
                                                        match c.to_string().parse::<u8>() {
                                                            // 数字の連続
                                                            Ok(u) => {
                                                                nums.push(u);
                                                                continue;
                                                            }
                                                            // エラー: 全く関係のない文字
                                                            Err(_) => {
                                                                return Err(format_err!("不正な文字があります: {} in {}",c,s));
                                                            }
                                                        }
                                                    };
                                                tiles.append(&mut tiles_tmp);
                                                break;
                                            }
                                            continue;
                                        }
                                        // エラー(`123s4W`など)
                                        Err(_) => {
                                            return Err(format_err!("不正な文字があります: {} in {}",c,s));
                                        }
                                    }
                                }
                            }
                            None => {
                                return Err(format_err!("入力に不足があります: {}",s));
                            }
                        }
                    }
                    Err(_) => {
                        if Tile::east_markers().contains(&c) {
                            Wind::East.tile()
                        } else if Tile::south_markers().contains(&c) {
                            Wind::South.tile()
                        } else if Tile::west_markers().contains(&c) {
                            Wind::West.tile()
                        } else if Tile::north_markers().contains(&c) {
                            Wind::North.tile()
                        } else if Tile::white_markers().contains(&c) {
                            Dragon::White.tile()
                        } else if Tile::green_markers().contains(&c) {
                            Dragon::Green.tile()
                        } else if Tile::red_markers().contains(&c) {
                            Dragon::Red.tile()
                        } else {
                            return Err(format_err!("入力が不正です: {} in {}",c,s));
                        }
                    }
                };
            tiles.push(tile);
        }

        Ok(TilesNewType(tiles))
    }
}

impl Display for TilesNewType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let TilesNewType(vec) = &self;
        for v in vec {
            std::fmt::Display::fmt(v, f)?;
        }
        Ok(())
    }
}

pub struct DisplayVec<T: Display + Debug>(pub Vec<T>);

impl<T: Display + Debug> Display for DisplayVec<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let DisplayVec(vec) = &self;

        for v in vec {
            std::fmt::Display::fmt(v, f)?;
            writeln!(f, "")?;
        }

        Ok(())
    }
}

/// 手牌という概念
#[derive(Debug, Clone)]
pub struct Hand {
    /// 手牌(晒していない手牌)
    pub tiles: Vec<Tile>,
    /// 鳴きで成立した面子
    pub open_sets: Vec<OpenSet>,
    /// 当たり牌
    pub winning: Tile,
}

impl Hand {
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

impl FromStr for Hand {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 手牌
        let mut tiles = Vec::with_capacity(14);
        let mut open_sets = Vec::with_capacity(4);

        // パース
        let mut iter = s.chars().peekable();
        let mut store_tmp = String::new();
        while let Some(c) = iter.next() {
            if c == '[' {
                // これまでの並びを登録
                if store_tmp.len() != 0 {
                    let TilesNewType(mut tiles_tmp) = TilesNewType::from_str(&store_tmp)?;
                    tiles.append(&mut tiles_tmp);
                }
                // 鳴き成立の面子の譜面を読み取る
                let mut chars = Vec::new();
                while let Some(c) = iter.next() {
                    if c == ']' { break; }
                    chars.push(c);
                }
                store_tmp = chars.iter().collect();
                // 登録
                open_sets.push(OpenSet::from_str(&store_tmp)?);
                // 一時変数を初期化
                store_tmp = String::new();
            } else if c == '(' {
                // これまでの並びを登録
                if store_tmp.len() != 0 {
                    let TilesNewType(mut tiles_tmp) = TilesNewType::from_str(&store_tmp)?;
                    tiles.append(&mut tiles_tmp);
                }
                // 鳴き成立の面子の譜面を読み取る
                let mut chars = Vec::new();
                while let Some(c) = iter.next() {
                    if c == ')' { break; }
                    chars.push(c);
                }
                store_tmp = chars.iter().collect();
                let TilesNewType(mut tiles_tmp) = TilesNewType::from_str(&store_tmp)?;
                // 登録
                open_sets.push(OpenSet::ConcealedKong(tiles_tmp));
                // 一時変数を初期化
                store_tmp = String::new();
            } else {
                store_tmp.push(c);
            }
        }
        if store_tmp.len() != 0 {
            let TilesNewType(mut tiles_tmp) = TilesNewType::from_str(&store_tmp)?;
            tiles.append(&mut tiles_tmp);
        }

        // 少牌or多牌
        if tiles.len() + 3 * open_sets.len() < 14 {
            return Err(format_err!("少牌です: {}",s));
        } else if tiles.len() + 3 * open_sets.len() > 14 {
            return Err(format_err!("多牌です: {}",s));
        }

        // 当たり牌
        let winning = tiles.last().unwrap().clone();

        // ソート
        tiles.sort();

        Ok(Hand { tiles, open_sets, winning })
    }
}

/// 面子
pub trait Sets {
    fn fu(&self) -> Fu;

    fn all_character(&self) -> bool;

    fn all_circle(&self) -> bool;

    fn all_bamboo(&self) -> bool;

    fn all_honor(&self) -> bool;

    fn consists_of(&self, tiles: &Vec<Tile>) -> bool {
        self.vec().iter().all(|t| {
            tiles.contains(t)
        })
    }

    fn vec(&self) -> Vec<Tile>;
}

/// 鳴きで成立した面子
#[derive(Debug, Clone, PartialEq)]
pub enum OpenSet {
    /// ポン
    Pung(Vec<Tile>),
    /// チー
    Chow(Vec<Tile>),
    /// 明槓
    Kong(Vec<Tile>),
    /// 暗槓
    ConcealedKong(Vec<Tile>),
}

impl FromStr for OpenSet {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 中身
        let TilesNewType(mut vec) = TilesNewType::from_str(&s)?;
        vec.sort();

        // validation
        match vec.len() {
            3 => {
                if vec.is_flat() {
                    // 刻子
                    Ok(OpenSet::Pung(vec))
                } else if vec.is_sequential() {
                    // 順子
                    Ok(OpenSet::Chow(vec))
                } else {
                    Err(format_err!("入力が不正です: [{}]", s))
                }
            }
            4 => {
                if vec.is_flat() {
                    // 明槓 (暗槓はHand.parse()時に判断する)
                    Ok(OpenSet::Kong(vec))
                } else {
                    Err(format_err!("入力が不正です: [{}]", s))
                }
            }
            _ => {
                Err(format_err!("入力が不正です: [{}]", s))
            }
        }
    }
}

impl Display for OpenSet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            OpenSet::Pung(tiles) => {
                write!(f, "[")?;
                TilesNewType(tiles.clone()).fmt(f)?;
                write!(f, "]")
            }
            OpenSet::Chow(tiles) => {
                write!(f, "[")?;
                TilesNewType(tiles.clone()).fmt(f)?;
                write!(f, "]")
            }
            OpenSet::Kong(tiles) => {
                write!(f, "[")?;
                TilesNewType(tiles.clone()).fmt(f)?;
                write!(f, "]")
            }
            OpenSet::ConcealedKong(tiles) => {
                write!(f, "(")?;
                TilesNewType(tiles.clone()).fmt(f)?;
                write!(f, ")")
            }
        }
    }
}

impl Sets for OpenSet {
    fn fu(&self) -> Fu {
        match &self {
            OpenSet::Pung(vec) => {
                Fu(if vec.all_yaotyu() {
                    4
                } else { 2 })
            }
            OpenSet::ConcealedKong(vec) => {
                Fu(if vec.all_yaotyu() {
                    32
                } else { 16 })
            }
            OpenSet::Kong(vec) => {
                Fu(if vec.all_yaotyu() {
                    16
                } else { 8 })
            }
            _ => Fu(0)
        }
    }

    fn all_character(&self) -> bool {
        let vec = match &self {
            OpenSet::Chow(vec) => vec,
            OpenSet::Pung(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Character(_) => true,
            _ => false,
        })
    }

    fn all_circle(&self) -> bool {
        let vec = match &self {
            OpenSet::Chow(vec) => vec,
            OpenSet::Pung(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Circle(_) => true,
            _ => false,
        })
    }

    fn all_bamboo(&self) -> bool {
        let vec = match &self {
            OpenSet::Chow(vec) => vec,
            OpenSet::Pung(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Bamboo(_) => true,
            _ => false,
        })
    }

    fn all_honor(&self) -> bool {
        let vec = match &self {
            OpenSet::Chow(vec) => vec,
            OpenSet::Pung(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Honour(_) => true,
            _ => false,
        })
    }

    fn vec(&self) -> Vec<Tile> {
        match &self {
            OpenSet::Chow(vec) => vec,
            OpenSet::Pung(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.clone()
    }
}

impl Tiles for OpenSet {
    fn contains_yaotyu(&self) -> bool {
        match &self {
            OpenSet::Pung(vec) => vec,
            OpenSet::Chow(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.contains_yaotyu()
    }

    fn all_yaotyu(&self) -> bool {
        match &self {
            OpenSet::Pung(vec) => vec,
            OpenSet::Chow(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.all_yaotyu()
    }

    fn contains_terminal(&self) -> bool {
        match &self {
            OpenSet::Pung(vec) => vec,
            OpenSet::Chow(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.contains_terminal()
    }

    fn all_terminal(&self) -> bool {
        match &self {
            OpenSet::Pung(vec) => vec,
            OpenSet::Chow(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.all_terminal()
    }


    fn is_sequential(&self) -> bool {
        match &self {
            OpenSet::Pung(vec) => vec,
            OpenSet::Chow(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.is_sequential()
    }

    fn is_flat(&self) -> bool {
        match &self {
            OpenSet::Pung(vec) => vec,
            OpenSet::Chow(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.is_flat()
    }

    fn count(&self, tile: &Tile) -> u8 {
        match &self {
            OpenSet::Pung(vec) => vec,
            OpenSet::Chow(vec) => vec,
            OpenSet::Kong(vec) => vec,
            OpenSet::ConcealedKong(vec) => vec,
        }.count(tile)
    }

    fn sum_tile(&self) -> Option<Tile> {
        match &self {
            OpenSet::Pung(vec) => vec.clone(),
            OpenSet::Chow(vec) => vec.clone(),
            OpenSet::Kong(vec) => {
                let mut vec = vec.clone();
                vec.remove(0);
                vec
            }
            OpenSet::ConcealedKong(vec) => {
                let mut vec = vec.clone();
                vec.remove(0);
                vec
            }
        }.sum_tile()
    }
}

/// 手牌の中で成立した面子
#[derive(Debug, Clone, PartialEq)]
pub enum Set {
    /// 順子
    Chow(Vec<Tile>),
    /// 刻子
    Pung(Vec<Tile>),
    /// 対子
    Pair(Vec<Tile>),
}

impl Set {
    pub fn new(vec: Vec<Tile>) -> Result<Self, failure::Error> {
        if vec.is_sequential() {
            Ok(Chow(vec))
        } else if vec.is_flat() {
            if vec.len() == 2 {
                Ok(Pair(vec))
            } else {
                Ok(Pung(vec))
            }
        } else {
            Err(format_err!("面子ではありません: {:?}",vec))
        }
    }
}

impl Sets for Set {
    fn fu(&self) -> Fu {
        match &self {
            Set::Pung(vec) => {
                Fu(if vec.all_yaotyu() {
                    8
                } else { 4 })
            }
            _ => Fu(0)
        }
    }

    fn all_character(&self) -> bool {
        let vec = match &self {
            Set::Chow(vec) => vec,
            Set::Pung(vec) => vec,
            Set::Pair(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Character(_) => true,
            _ => false,
        })
    }

    fn all_circle(&self) -> bool {
        let vec = match &self {
            Set::Chow(vec) => vec,
            Set::Pung(vec) => vec,
            Set::Pair(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Circle(_) => true,
            _ => false,
        })
    }

    fn all_bamboo(&self) -> bool {
        let vec = match &self {
            Set::Chow(vec) => vec,
            Set::Pung(vec) => vec,
            Set::Pair(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Bamboo(_) => true,
            _ => false,
        })
    }

    fn all_honor(&self) -> bool {
        let vec = match &self {
            Set::Chow(vec) => vec,
            Set::Pung(vec) => vec,
            Set::Pair(vec) => vec,
        };
        vec.iter().all(|tile| match tile {
            Tile::Honour(_) => true,
            _ => false,
        })
    }

    fn vec(&self) -> Vec<Tile> {
        match &self {
            Set::Chow(vec) => vec,
            Set::Pung(vec) => vec,
            Set::Pair(vec) => vec,
        }.clone()
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let tiles = match &self {
            Set::Pung(tiles) => tiles,
            Set::Chow(tiles) => tiles,
            Set::Pair(tiles) => tiles,
        }.clone();
        TilesNewType(tiles).fmt(f)
    }
}

impl Tiles for Set {
    fn contains_yaotyu(&self) -> bool {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.contains_yaotyu()
    }

    fn all_yaotyu(&self) -> bool {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.all_yaotyu()
    }

    fn contains_terminal(&self) -> bool {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.contains_terminal()
    }

    fn all_terminal(&self) -> bool {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.all_terminal()
    }


    fn is_sequential(&self) -> bool {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.is_sequential()
    }

    fn is_flat(&self) -> bool {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.is_flat()
    }

    fn count(&self, tile: &Tile) -> u8 {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.count(tile)
    }

    fn sum_tile(&self) -> Option<Tile> {
        match &self {
            Chow(vec) => vec,
            Pung(vec) => vec,
            Pair(vec) => vec,
        }.sum_tile()
    }
}