use std::fmt::{Display, Formatter, Error};

/// 牌
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub enum Tile {
    /// 萬子
    Character(u8),
    /// 筒子
    Circle(u8),
    /// 索子
    Bamboo(u8),
    /// 字牌
    Honour(Honour),
}

impl Tile {
    pub fn characters_markers() -> Vec<char> {
        vec!['m', '萬']
    }

    pub fn circles_markers() -> Vec<char> {
        vec!['p', '筒']
    }

    pub fn bamboos_markers() -> Vec<char> {
        vec!['s', '索']
    }

    pub fn east_markers() -> Vec<char> {
        vec!['E', '東']
    }

    pub fn south_markers() -> Vec<char> {
        vec!['S', '南']
    }

    pub fn west_markers() -> Vec<char> {
        vec!['W', '西']
    }

    pub fn north_markers() -> Vec<char> {
        vec!['N', '北']
    }

    pub fn white_markers() -> Vec<char> {
        vec!['D', 'P', '白']
    }

    pub fn green_markers() -> Vec<char> {
        vec!['H', 'F', '發', '発']
    }

    pub fn red_markers() -> Vec<char> {
        vec!['T', 'C', '中']
    }

    pub fn next(&self) -> Option<Tile> {
        match self {
            Tile::Character(u) => {
                if (1..9).contains(u) {
                    Some(Tile::Character(u + 1))
                } else {
                    None
                }
            }
            Tile::Circle(u) => {
                if (1..9).contains(u) {
                    Some(Tile::Circle(u + 1))
                } else {
                    None
                }
            }
            Tile::Bamboo(u) => {
                if (1..9).contains(u) {
                    Some(Tile::Bamboo(u + 1))
                } else {
                    None
                }
            }
            _ => None
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let jorker = "🀪";
        let characters = vec!["🀪", "🀇", "🀈", "🀉", "🀊", "🀋", "🀌", "🀍", "🀎", "🀏"];
        let circles = vec!["🀪", "🀙", "🀚", "🀛", "🀜", "🀝", "🀞", "🀟", "🀠", "🀡"];
        let bamboos = vec!["🀪", "🀐", "🀑", "🀒", "🀓", "🀔", "🀕", "🀖", "🀗", "🀘"];

        match &self {
            Tile::Character(u) => {
                match characters.get(*u as usize) {
                    Some(img) => {
                        write!(f, "{}", img)
                    }
                    None => {
                        write!(f, "{}", jorker)
                    }
                }
            }
            Tile::Circle(u) => {
                match circles.get(*u as usize) {
                    Some(img) => {
                        write!(f, "{}", img)
                    }
                    None => {
                        write!(f, "{}", jorker)
                    }
                }
            }
            Tile::Bamboo(u) => {
                match bamboos.get(*u as usize) {
                    Some(img) => {
                        write!(f, "{}", img)
                    }
                    None => {
                        write!(f, "{}", jorker)
                    }
                }
            }
            Tile::Honour(honour) => { honour.fmt(f) }
        }
    }
}


/// 字牌
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub enum Honour {
    /// 風牌
    Wind(Wind),
    /// 三元牌
    Dragon(Dragon),
}

impl Display for Honour {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Honour::Wind(winds) => winds.fmt(f),
            Honour::Dragon(dragons) => dragons.fmt(f),
        }
    }
}

/// 風牌
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

impl Display for Wind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Wind::East => write!(f, "🀀"),
            Wind::South => write!(f, "🀁"),
            Wind::West => write!(f, "🀂"),
            Wind::North => write!(f, "🀃"),
        }
    }
}

impl Wind {
    pub fn tile(self) -> Tile {
        Tile::Honour(Honour::Wind(self))
    }
}

/// 三元牌
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub enum Dragon {
    White,
    Green,
    Red,
}

impl Display for Dragon {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Dragon::White => write!(f, "🀆"),
            Dragon::Green => write!(f, "🀅"),
            Dragon::Red => write!(f, "🀄"),
        }
    }
}

impl Dragon {
    pub fn tile(self) -> Tile {
        Tile::Honour(Honour::Dragon(self))
    }
}

impl Tile {
    #![allow(dead_code)]
    /// 数牌が否か
    pub fn is_suits(&self) -> bool {
        match self {
            Tile::Honour(_) => false,
            _ => true,
        }
    }

    /// 字牌か否か
    pub fn is_honours(&self) -> bool {
        !self.is_suits()
    }

    /// 端牌か否か
    pub fn is_terminal(&self) -> bool {
        if self.is_suits() {
            let num;
            match self {
                Tile::Character(u) => { num = u; }
                Tile::Circle(u) => { num = u; }
                Tile::Bamboo(u) => { num = u; }
                _ => unreachable!()
            }

            num == &1 || num == &9
        } else {
            false
        }
    }

    /// 么九牌か否か
    pub fn is_yaotyu(&self) -> bool {
        self.is_honours() || self.is_terminal()
    }

    /// 中張牌か否か
    pub fn is_simple(&self) -> bool {
        !self.is_yaotyu()
    }
}
