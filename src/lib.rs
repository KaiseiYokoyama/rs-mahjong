#[macro_use]
extern crate failure;

mod tiles;
mod parse;
mod groups;
mod yaku;
mod evaluate;
mod score;
pub mod calculator;

pub use calculator::calc;

#[cfg(test)]
mod tests {
    use crate::tiles::{Tile, Dragon, Wind};
    use crate::groups::*;

    #[test]
    fn sort_tiles() {
        let wanzu = Tile::Character(1);
        let pinzu = Tile::Circle(1);
        let souzu = Tile::Bamboo(1);
        let fonpai = Wind::East.tile();
        let sangenpai = Dragon::White.tile();

        let mut vec = vec![sangenpai.clone(), fonpai.clone(), souzu.clone(), pinzu.clone(), wanzu.clone()];
        vec.sort();

        assert_eq!(vec, vec![wanzu, pinzu, souzu, fonpai, sangenpai]);
    }

    #[test]
    fn from_str() -> Result<(), failure::Error> {
        let hand = Hand::from_str("1s1s1s1s2s3s4s5s6s7s8s9s9s9s")?;
        let vec = vec![
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(2),
            Tile::Bamboo(3),
            Tile::Bamboo(4),
            Tile::Bamboo(5),
            Tile::Bamboo(6),
            Tile::Bamboo(7),
            Tile::Bamboo(8),
            Tile::Bamboo(9),
            Tile::Bamboo(9),
            Tile::Bamboo(9)];
        assert_eq!(hand.tiles(), &vec);
        Ok(())
    }

    #[test]
    fn from_str2() -> Result<(), failure::Error> {
        let hand = Hand::from_str("11112345678999s")?;
        let vec = vec![
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(2),
            Tile::Bamboo(3),
            Tile::Bamboo(4),
            Tile::Bamboo(5),
            Tile::Bamboo(6),
            Tile::Bamboo(7),
            Tile::Bamboo(8),
            Tile::Bamboo(9),
            Tile::Bamboo(9),
            Tile::Bamboo(9)];
        assert_eq!(hand.tiles(), &vec);
        Ok(())
    }

    #[test]
    fn from_str3() -> Result<(), failure::Error> {
        let hand = Hand::from_str("11112345678s東東東")?;
        let vec = vec![
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(1),
            Tile::Bamboo(2),
            Tile::Bamboo(3),
            Tile::Bamboo(4),
            Tile::Bamboo(5),
            Tile::Bamboo(6),
            Tile::Bamboo(7),
            Tile::Bamboo(8),
            Wind::East.tile(),
            Wind::East.tile(),
            Wind::East.tile()];
        assert_eq!(hand.tiles(), &vec);
        Ok(())
    }
}
