pub trait YakuAttributes {
    fn name(&self) -> String;
}

pub mod situation {
    use crate::yaku::YakuAttributes;
    use crate::score::Han;

    /// 状況役
    #[derive(Debug)]
    pub struct SituationYaku {
        /// 名前
        name: String,
        /// 飜数
        han_value: Han,
    }

    impl SituationYaku {
        #![allow(dead_code)]
        pub fn new(name: &str, han_value: u32) -> Self {
            Self { name: name.to_string(), han_value: Han(han_value) }
        }

        pub fn han_value(&self) -> Han {
            self.han_value.clone()
        }

        pub fn ready() -> Self {
            Self::new("立直 / Ready hand", 1)
        }

        pub fn nagashi_mangan() -> Self {
            Self::new("流し満貫 / Nagashi mangan", 4)
        }

        pub fn self_pick() -> Self {
            Self::new("門前清自摸和 / Self-pick", 1)
        }

        pub fn one_shot() -> Self {
            Self::new("一発 / One-shot", 1)
        }

        pub fn last_tile_from_the_wall() -> Self {
            Self::new("海底摸月 / Last tile from the wall", 1)
        }

        pub fn last_discard() -> Self {
            Self::new("河底撈魚 / Last discard", 1)
        }

        pub fn dead_wall_draw() -> Self {
            Self::new("嶺上開花 / Dead wall draw", 1)
        }

        pub fn robbing_a_quad() -> Self {
            Self::new("槍槓 / Robbing a quad", 1)
        }

        pub fn double_ready() -> Self {
            Self::new("ダブル立直 / Double ready", 2)
        }
    }

    impl YakuAttributes for SituationYaku {
        fn name(&self) -> String {
            self.name.to_string()
        }
    }
}

pub mod hand {
    use crate::yaku::YakuAttributes;
    use crate::evaluate::Wait;
    use crate::score::{Han, Fu};
    use crate::tiles::Tile;

    /// 手役
    pub struct HandYaku {
        /// 名前
        pub name: String,
        /// ルール
        pub rule: Box<Fn(&Wait) -> Option<Han>>,
        /// 下位役
        pub sub: Option<Box<HandYaku>>,
        /// 府数(平和、七対子対応)
        pub fu: Option<Box<Fn(&bool) -> Fu>>,
    }

    impl HandYaku {
        pub fn new(name: &str, sub: Option<Box<HandYaku>>, rule: Box<Fn(&Wait) -> Option<Han>>, fu: Option<Box<Fn(&bool) -> Fu>>) -> Self {
            HandYaku { name: name.to_string(), sub, rule, fu }
        }
    }

    impl YakuAttributes for HandYaku {
        fn name(&self) -> String {
            self.name.to_string()
        }
    }

    pub struct Yakuman {
        pub name: String,
        /// ルール
        pub rule: Box<Fn(&Wait, &Vec<Tile>) -> u32>,
        /// 下位役
        pub sub: Option<Box<Yakuman>>,
    }

    impl Yakuman {
        pub fn new(name: &str, rule: Box<Fn(&Wait, &Vec<Tile>) -> u32>, sub: Option<Box<Yakuman>>) -> Self {
            Yakuman { name: name.to_string(), rule, sub }
        }
    }

    impl YakuAttributes for Yakuman {
        fn name(&self) -> String {
            self.name.to_string()
        }
    }
}
