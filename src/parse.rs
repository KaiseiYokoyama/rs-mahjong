use crate::groups::{Hand, OpenSet, Set, Tiles, TilesNewType};
use crate::tiles::Tile;
use std::fmt::{Display, Formatter, Error};

/// 手牌をパースする
#[derive(Debug)]
pub struct ParsedHand {
    /// 手牌(晒した牌を含む)
    pub tiles: Vec<Tile>,
    /// 最終形候補
    pub nodes: Vec<Node>,
    /// 当たり牌
    pub winning: Tile,
}

impl ParsedHand {
    pub fn new(hand: &Hand) -> Self {
        let mut tiles = hand.tiles.clone();
        hand.open_sets.iter().for_each(|open| {
            let mut vec = match open {
                OpenSet::Pung(vec) => vec,
                OpenSet::Chow(vec) => vec,
                OpenSet::Kong(vec) => vec,
                OpenSet::ConcealedKong(vec) => vec,
            }.clone();
            tiles.append(&mut vec);
        });

        // 面子の候補を生成
        let nodes = Root::new(hand).search_leafs();

        if nodes.len() == 0 {
            // 七対子判定
            let mut tiles = hand.tiles.clone();
            tiles.sort();
            let mut tiles_iter = tiles.iter();

            let mut sets = Vec::<Set>::new();
            while let Some(tile) = tiles_iter.next() {
                if let Some(tile2) = tiles_iter.next() {
                    if tile == tile2 {
                        let set = Set::new(vec![tile.clone(), tile2.clone()]);
                        match set {
                            Ok(set) => {
                                sets.push(set);
                            }
                            Err(_) => { break; }
                        }
                    }
                }
            }
            if sets.len() == 7 {
                let node = Node { remaining: Vec::new(), open_sets: Vec::new(), sets, pong: Box::new(None), chow: Box::new(None) };
                return ParsedHand { tiles, nodes: vec![node], winning: hand.winning.clone() };
            }
        }

        ParsedHand { tiles, nodes, winning: hand.winning.clone() }
    }
}

impl Display for ParsedHand {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "input: ")?;
        TilesNewType(self.tiles.clone()).fmt(f)?;
        // 改行
        writeln!(f, "")?;
        self.nodes.iter().try_for_each(|node| {
            std::fmt::Display::fmt(node, f)?;
            writeln!(f, "")
        })
    }
}

#[derive(Debug)]
pub struct Root {
    nodes: Vec<Node>
}

impl Root {
    pub fn new(hand: &Hand) -> Self {
        let mut root = Root {
            nodes: Vec::with_capacity(7),
        };
        let mut heads = Vec::new();
        let tiles = hand.tiles().clone();

        for tile in tiles {
            let mut tiles = hand.tiles().clone();
            let head0 = tiles.iter().position(|t| t == &tile).unwrap();
            let head0 = tiles.remove(head0);
            let head1 = match tiles.iter().position(|t| t == &tile) {
                // 対子成立
                Some(head1) => {
                    head1
                }
                // 対子成立せず
                None => { continue; }
            };
            let head1 = tiles.remove(head1);
            // 処理済みの雀頭
            if heads.contains(&(head1.clone(), tiles.clone())) {
                continue;
            }
            heads.push((head1.clone(), tiles.clone()));


            let head = Set::new(vec![head0, head1]).unwrap();
            let remaining = tiles;

            let mut sets = Vec::with_capacity(6);
            sets.push(head);

            let leaf = Node::new(remaining, hand.open_sets.clone(), sets);
            match leaf {
                Ok(leaf_ok) => {
                    root.nodes.push(leaf_ok);
                }
                _ => {}
            }
        }

        // 国士
        if heads.len() == 1 {
            let (head, tiles) = heads.get(0).unwrap();
            let head = Set::Pair(vec![head.clone(), head.clone()]);
            let body = Set::Chow(tiles.clone());
            let node = Node { remaining: Vec::with_capacity(0), open_sets: Vec::with_capacity(0), sets: vec![head, body], pong: Box::new(None), chow: Box::new(None) };
            root.nodes.push(node);
        }

        root
    }
    pub fn search_leafs(&self) -> Vec<Node> {
        let mut leafs = Vec::new();
        for node in &self.nodes {
            leafs.append(&mut node.search_leafs());
        }
        leafs
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    /// 未処理の手牌
    pub remaining: Vec<Tile>,
    /// 鳴いて成立した面子
    pub open_sets: Vec<OpenSet>,
    /// 手牌で成立している面子
    pub sets: Vec<Set>,
    /// 子ノード
    pub pong: Box<Option<Node>>,
    /// 子ノード
    pub chow: Box<Option<Node>>,
}

impl Node {
    fn pong(&self) -> &Box<Option<Node>> {
        &self.pong
    }
    fn new(remaining: Vec<Tile>, open_sets: Vec<OpenSet>, sets: Vec<Set>) -> Result<Node, failure::Error> {
        let (mut pong, mut chow) = (None, None);

        if remaining.len() == 0 {
            return Ok(Node { remaining, open_sets, sets, pong: Box::new(pong), chow: Box::new(chow) });
        }
        let first = remaining[0].clone();

        // 刻子が取れるか？
        if remaining.count(&first) >= 3 {

            // 刻子に使う牌を削除
            let mut tiles = remaining.clone();
            let head0 = tiles.iter().position(|t| t == &first).unwrap();
            let head0 = tiles.remove(head0);
            let head1 = tiles.iter().position(|t| t == &first).unwrap();
            let head1 = tiles.remove(head1);
            let head2 = tiles.iter().position(|t| t == &first).unwrap();
            let head2 = tiles.remove(head2);

            let set = Set::new(vec![head0, head1, head2])?;
            let mut sets = sets.clone();
            sets.push(set);

            pong = Node::new(tiles, open_sets.clone(), sets).ok();
        }
        // 順子が取れるか？
        match first.next() {
            Some(second) => match second.next() {
                Some(third) => {
                    if remaining.contains(&second) & &remaining.contains(&third) {

                        // 刻子に使う牌を削除
                        let mut tiles = remaining.clone();
                        let head0 = tiles.iter().position(|t| t == &first).unwrap();
                        let head0 = tiles.remove(head0);
                        let head1 = tiles.iter().position(|t| t == &second).unwrap();
                        let head1 = tiles.remove(head1);
                        let head2 = tiles.iter().position(|t| t == &third).unwrap();
                        let head2 = tiles.remove(head2);

                        let set = Set::new(vec![head0, head1, head2])?;
                        let mut sets = sets.clone();
                        sets.push(set);

                        chow = Node::new(tiles, open_sets.clone(), sets).ok();
                    }
                }
                _ => {}
            }
            _ => {}
        }

        if !remaining.is_empty() && (&pong, &chow) == (&None, &None) {
            return Err(format_err!("parse error"));
        }

        return Ok(Node { remaining, open_sets, sets, pong: Box::new(pong), chow: Box::new(chow) });
    }
    fn search_leafs(&self) -> Vec<Node> {
        if self.pong.is_none() & &self.chow.is_none() {
            return vec![self.clone()];
        }
        let mut vec = Vec::new();
        &self.pong.clone().map(|ref node| vec.append(&mut node.search_leafs()));
        &self.chow.clone().map(|ref node| vec.append(&mut node.search_leafs()));
        vec
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // 手牌の面子を出力
        for set in &self.sets {
            set.fmt(f)?;
            // スペース
            write!(f, " ")?;
        }

        // スペース
        write!(f, " ")?;

        // 晒した面子を出力
        for open_set in &self.open_sets {
            open_set.fmt(f)?;
            // スペース
            write!(f, " ")?;
        }

        Ok(())
    }
}

