use std::fmt::{Display, Formatter, Error};
use std::ops::{Add, AddAssign};
use std::cmp::Ordering;

/// 得点
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Score {
    Mangan {
        han: Han,
    },
    Haneman {
        han: Han,
    },
    Baiman {
        han: Han,
    },
    Sanbaiman {
        han: Han,
    },
    Yakuman,
    KazoeYakuman {
        han: Han,
    },
    MultipleYakuman {
        multiple: u8,
    },
    Other {
        han: Han,
        fu: Fu,
    },
}

impl Score {
    pub fn new(han: Han, fu: Fu) -> Self {
        let Han(han_value) = han;
        if han_value >= 13 as u32 {
            Score::KazoeYakuman { han }
        } else if (11..13).contains(&han_value) {
            Score::Sanbaiman { han }
        } else if (8..11).contains(&han_value) {
            Score::Baiman { han }
        } else if (6..8).contains(&han_value) {
            Score::Haneman { han }
        } else if (4..6).contains(&han_value) {
            Score::Mangan { han }
        } else {
            Score::Other { han, fu }
        }
    }

    pub fn yakuman(multiple: u8) -> Self {
        if multiple == 1 {
            Score::Yakuman
        } else {
            Score::MultipleYakuman { multiple }
        }
    }

    pub fn jp_name(&self) -> String {
        match &self {
            Score::Other { .. } => { String::new() }
            Score::Mangan { .. } => { "満貫".to_string() }
            Score::Haneman { .. } => { "跳満".to_string() }
            Score::Baiman { .. } => { "倍満".to_string() }
            Score::Sanbaiman { .. } => { "三倍満".to_string() }
            Score::Yakuman => { "役満".to_string() }
            Score::KazoeYakuman { .. } => { "数え役満".to_string() }
            Score::MultipleYakuman { multiple } => {
                if multiple > &(3 as u8) {
                    "マルチ役満".to_string()
                } else if multiple == &3 {
                    "トリプル役満".to_string()
                } else if multiple == &2 {
                    "ダブル役満".to_string()
                } else {
                    "役満".to_string()
                }
            }
        }
    }

    pub fn en_name(&self) -> String {
        match &self {
            Score::Other { .. } => { String::new() }
            Score::Mangan { .. } => { "Mangan".to_string() }
            Score::Haneman { .. } => { "Haneman".to_string() }
            Score::Baiman { .. } => { "Baiman".to_string() }
            Score::Sanbaiman { .. } => { "Sanbaiman".to_string() }
            Score::Yakuman => { "Yakuman".to_string() }
            Score::KazoeYakuman { .. } => { "Kazoe Yakuman".to_string() }
            Score::MultipleYakuman { multiple } => {
                if multiple > &(3 as u8) {
                    "Multiple Yakuman".to_string()
                } else if multiple == &3 {
                    "Triple Yakuman".to_string()
                } else if multiple == &2 {
                    "Double Yakuman".to_string()
                } else {
                    "Yakuman".to_string()
                }
            }
        }
    }

    pub fn score(&self, is_dealer: bool) -> u32 {
        match &self {
            Score::Mangan { .. } => {
                if is_dealer {
                    12000
                } else { 8000 }
            }
            Score::Haneman { .. } => {
                if is_dealer {
                    18000
                } else { 12000 }
            }
            Score::Baiman { .. } => {
                if is_dealer {
                    24000
                } else { 16000 }
            }
            Score::Sanbaiman { .. } => {
                if is_dealer {
                    36000
                } else { 24000 }
            }
            Score::KazoeYakuman { .. } | Score::Yakuman => {
                if is_dealer {
                    48000
                } else { 36000 }
            }
            Score::MultipleYakuman { multiple } => {
                let score =
                    if is_dealer {
                        48000
                    } else { 36000 }
                        * multiple.clone() as u32;
                score
            }
            Score::Other { han, fu } => {
                let Fu(fu) = fu;
                let fu = ((fu + (10 - 1)) / 10) * 10;
                let Han(han) = han;

                let score =
                    if is_dealer { 6 as u32 } else { 4 as u32 }
                        * fu * u32::pow(2, han.clone() + 2);
                score
            }
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            self.score(false).partial_cmp(&other.score(false))
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let score = self.score(false);
        let score_dealer = self.score(true);
        match &self {
            Score::Other { han, fu } => {
                let score_display = ((score + (100 - 1)) / 100) * 100;
                let score_draw_dealer = ((score / 2 + (100 - 1)) / 100) * 100;
                let score_draw = ((score / 4 + (100 - 1)) / 100) * 100;

                let score_dealer_display = ((score_dealer + (100 - 1)) / 100) * 100;
                let score_dealer_draw = ((score_dealer / 3 + (100 - 1)) / 100) * 100;

                writeln!(f, "{}{}", fu, han)?;
                writeln!(f, "Non-Dealer: {} / {} - {}", score_display, score_draw, score_draw_dealer)?;
                writeln!(f, "Dealer: {} / {} ALL", score_dealer_display, score_dealer_draw)
            }
            _ => {
                writeln!(f, "{} / {}", &self.jp_name(), &self.en_name())?;
                writeln!(f, "Non-Dealer: {} / {} - {}", score, score / 4, score / 2)?;
                writeln!(f, "Dealer: {} / {} ALL", score_dealer, score_dealer / 3)
            }
        }
    }
}

/// 翻
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Han(pub u32);

impl Display for Han {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Han(han) = &self;
        write!(f, "{}翻", han)
    }
}

impl Add for Han {
    type Output = Han;

    fn add(self, rhs: Self) -> Self::Output {
        let (Han(origin), Han(rhs)) = (self, rhs);
        Han(origin + rhs)
    }
}

impl AddAssign for Han {
    fn add_assign(&mut self, rhs: Self) {
        let (Han(origin), Han(rhs)) = (self, rhs);
        *origin += rhs;
    }
}

/// 符
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fu(pub u32);

impl Display for Fu {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Fu(fu) = &self;
        write!(f, "{}符", fu)
    }
}

impl Add for Fu {
    type Output = Fu;

    fn add(self, rhs: Self) -> Self::Output {
        let (Fu(origin), Fu(rhs)) = (self, rhs);
        Fu(origin + rhs)
    }
}

impl AddAssign for Fu {
    fn add_assign(&mut self, rhs: Self) {
        let (Fu(origin), Fu(rhs)) = (self, rhs);
        *origin += rhs;
    }
}