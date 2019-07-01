use crate::parse::{Node, ParsedHand};
use crate::yaku::situation::SituationYaku;
use crate::tiles::{Tile, Dragon, Wind};
use crate::groups::{Tiles, OpenSet, Set, Sets, Hand};
use crate::yaku::hand::{HandYaku, Yakuman};
use crate::score::{Fu, Score, Han};
use crate::yaku::YakuAttributes;

use std::fmt::{Display, Formatter, Error};
pub use std::str::FromStr;

pub struct Evaluator {
    /// 確定している状況役(リーチ、ツモなど)
    situation: Vec<SituationYaku>,
    /// 採用されている役満
    adopted_yakuman_list: Vec<Yakuman>,
    /// 採用されている手役
    adopted_yaku_list: Vec<HandYaku>,
    /// 風
    prevalent_wind: Option<Tile>,
    /// 自風
    seat_wind: Option<Tile>,
    /// ドラ
    dora: Vec<Tile>,
    /// 裏ドラ
    ura_dora: Vec<Tile>,
}

impl Evaluator {
    pub fn new(prevalent_wind: Option<Tile>, seat_wind: Option<Tile>, dora: Vec<Tile>, ura_dora: Vec<Tile>) -> Self {
        let adopted_yaku_list = Self::default_adopted_yaku_list(&seat_wind, &prevalent_wind);
        let adopted_yakuman_list = Self::default_adopted_yakuman_list();

        Self { situation: Vec::new(), adopted_yakuman_list, adopted_yaku_list, prevalent_wind, seat_wind, dora, ura_dora }
    }

    pub fn default_adopted_yaku_list(seat_wind: &Option<Tile>, prevalent_wind: &Option<Tile>) -> Vec<HandYaku> {
        let (seat_wind, prevalent_wind) = (seat_wind.clone(), prevalent_wind.clone());
        let nopoints = HandYaku::new("平和 / No-points hand", None,
                                     Box::new(|candidate: &Wait| {
                                         if !candidate.closed() { return None; }
                                         match candidate {
                                             Wait::Ryanmen(_, fu, _) => {
                                                 if fu == &Fu(30) {
                                                     Some(Han(1))
                                                 } else { None }
                                             }
                                             _ => None
                                         }
                                     }),
                                     Some(Box::new(|draw: &bool| {
                                         if draw.clone() { Fu(20) } else { Fu(30) }
                                     })));
        let oneset =
            HandYaku::new("一盃口 / One set of identical sequences", None,
                          Box::new(|candidate: &Wait| {
                              if !candidate.closed() { return None; }
                              let sets = candidate.node().clone().sets;
                              // 重複を調べる
                              let mut chows: Vec<Set> = Vec::new();
                              sets.iter().for_each(|set|
                                  match set {
                                      Set::Chow(_) => chows.push(set.clone()),
                                      _ => {}
                                  });
                              if chows.iter().any(|set| sets.iter().filter(|set_| set_ == &set).count() == 2) {
                                  Some(Han(1))
                              } else { None }
                          }), None);
        let twoset =
            HandYaku::new("二盃口 / Two set of identical sequences", Some(Box::new(oneset)),
                          Box::new(|candidate: &Wait| {
                              if !candidate.closed() { return None; }
                              let sets = candidate.node().clone().sets;
                              // 重複を調べる
                              let mut chows: Vec<Set> = Vec::new();
                              sets.iter().for_each(|set|
                                  match set {
                                      Set::Chow(_) => {
                                          chows.push(set.clone())
                                      }
                                      _ => {}
                                  });
                              // vec.iter().all()はiterの中身が無い場合trueになってしまう(七対子などが該当する)
                              if chows.len() != 4 { return None; }
                              if chows.iter().all(|set| sets.iter().filter(|set_| set_ == &set).count() == 2) {
                                  Some(Han(3))
                              } else { None }
                          }), None);
        let seven_pairs =
            HandYaku::new("七対子 / Seven pairs", None,
                          Box::new(|candidate: &Wait| {
                              if !candidate.closed() { return None; }
                              let sets = &candidate.node().sets;
                              // ７つの面子からなる
                              if sets.len() == 7
                                  &&
                                  // 全て対子である
                                  sets.iter().all(|set| match set {
                                      Set::Pair(_) => true,
                                      _ => false
                                  }) { Some(Han(2)) } else { None }
                          }), Some(Box::new(|_draw: &bool| { Fu(25) })));
        let all_simple =
            HandYaku::new("タンヤオ / All simple", None,
                          Box::new(|candidate: &Wait| {
                              let sets = &candidate.node().sets;
                              let open_sets = &candidate.node().open_sets;
                              // 手牌に么九牌がない
                              if sets.iter().all(|set| !set.contains_yaotyu())
                                  &&
                                  // 晒した牌にもない
                                  open_sets.iter().all(|open| !open.contains_yaotyu()) {
                                  Some(Han(1))
                              } else { None }
                          }), None);
        let three_colour_straight =
            HandYaku::new("三色同順 / Three colour straight", None,
                          Box::new(|candidate: &Wait| {
                              let mut chow_sums = Vec::new();
                              candidate.node().sets.iter().for_each(|set| {
                                  if set.is_sequential() {
                                      chow_sums.push(set.sum_tile().unwrap());
                                  }
                              });
                              candidate.node().open_sets.iter().for_each(|set| {
                                  if set.is_sequential() {
                                      chow_sums.push(set.sum_tile().unwrap());
                                  }
                              });
                              // 順子が3つ以上ないならfalse
                              if chow_sums.len() < 3 { return None; }
                              // すべての順子について、三色同順の構成要素になりうるかvalidation
                              if chow_sums.iter().any(|chow| {
                                  let sum = match chow {
                                      Tile::Character(u) => {
                                          u
                                      }
                                      Tile::Circle(u) => {
                                          u
                                      }
                                      Tile::Bamboo(u) => {
                                          u
                                      }
                                      _ => unreachable!()
                                  }.clone();
                                  let (mut character, mut circle, mut bamboo) = (false, false, false);
                                  for chow_sum in &chow_sums {
                                      if match chow_sum {
                                          Tile::Character(u) => {
                                              u
                                          }
                                          Tile::Circle(u) => {
                                              u
                                          }
                                          Tile::Bamboo(u) => {
                                              u
                                          }
                                          _ => unreachable!()
                                      }.clone() == sum {
                                          match chow_sum {
                                              Tile::Character(_) => {
                                                  character = true;
                                              }
                                              Tile::Circle(_) => {
                                                  circle = true;
                                              }
                                              Tile::Bamboo(_) => {
                                                  bamboo = true;
                                              }
                                              _ => unreachable!()
                                          }
                                      }
                                  }
                                  character && circle && bamboo
                              }) {
                                  Some(Han(if candidate.closed() { 2 } else { 1 }))
                              } else { None }
                          }), None);
        let straight =
            HandYaku::new("一気通貫 / Straight", None,
                          Box::new(|candidate: &Wait| {
                              let mut chow_sums = Vec::new();
                              candidate.node().sets.iter().for_each(|set| {
                                  if set.is_sequential() {
                                      chow_sums.push(set.sum_tile().unwrap());
                                  }
                              });
                              candidate.node().open_sets.iter().for_each(|set| {
                                  if set.is_sequential() {
                                      chow_sums.push(set.sum_tile().unwrap());
                                  }
                              });
                              // 順子が3つ以上ないならfalse
                              if chow_sums.len() < 3 { return None; }
                              // すべての順子について、一気通貫の構成要素になりうるかvalidation

                              if false ||
                                  chow_sums.contains(&Tile::Character(6))
                                      && chow_sums.contains(&Tile::Character(15))
                                      && chow_sums.contains(&Tile::Character(24)) ||
                                  chow_sums.contains(&Tile::Circle(6))
                                      && chow_sums.contains(&Tile::Circle(15))
                                      && chow_sums.contains(&Tile::Circle(24)) ||
                                  chow_sums.contains(&Tile::Bamboo(6))
                                      && chow_sums.contains(&Tile::Bamboo(15))
                                      && chow_sums.contains(&Tile::Bamboo(24)) {
                                  Some(Han(if candidate.closed() { 2 } else { 1 }))
                              } else { None }
                          }), None);
        let all_triplet_hand =
            HandYaku::new("対々和 / All triplet hand", None,
                          Box::new(|candidate: &Wait| {
                              if candidate.node().sets.len() == 7 {
                                  return None;
                              }
                              if candidate.node().sets.iter().all(|set| set.is_flat())
                                  && candidate.node().open_sets.iter().all(|set| set.is_flat()) {
                                  Some(Han(2))
                              } else { None }
                          }), None);
        let three_closed_triplets =
            HandYaku::new("三暗刻 / Three closed triplets", None,
                          Box::new(|candidate: &Wait| {
                              // 雀頭もis_flatがtrueになる点に注意
                              if candidate.node().sets.iter().filter(|set| set.is_flat()).count() +
                                  candidate.node().open_sets.iter().filter(|set| match set {
                                      OpenSet::ConcealedKong(_) => true,
                                      _ => false,
                                  }).count() == 3 + 1 {
                                  Some(Han(2))
                              } else { None }
                          }), None);
        let three_colour_triplets =
            HandYaku::new("三色同刻 / Three colour triplets", None,
                          Box::new(|candidate: &Wait| {
                              let mut pong_sums = Vec::new();
                              candidate.node().sets.iter().for_each(|set| {
                                  match set {
                                      Set::Pung(_) => {
                                          if let Some(tile) = set.sum_tile() {
                                              pong_sums.push(tile);
                                          }
                                      }
                                      _ => {}
                                  }
                              });
                              candidate.node().open_sets.iter().for_each(|set| {
                                  match set {
                                      OpenSet::Pung(_) | OpenSet::Kong(_) | OpenSet::ConcealedKong(_) => {
                                          if let Some(tile) = set.sum_tile() {
                                              pong_sums.push(tile);
                                          }
                                      }
                                      _ => {}
                                  }
                              });
                              // 刻子が3つ以上ないならfalse
                              if pong_sums.len() < 3 { return None; }
                              // すべての刻子について、三食同刻の構成要素になりうるかvalidation

                              if pong_sums.iter().any(|sum| match sum {
                                  Tile::Character(u) =>
                                      pong_sums.contains(&Tile::Circle(u.clone()))
                                          && pong_sums.contains(&Tile::Bamboo(u.clone())),
                                  Tile::Circle(u) =>
                                      pong_sums.contains(&Tile::Character(u.clone()))
                                          && pong_sums.contains(&Tile::Bamboo(u.clone())),
                                  Tile::Bamboo(u) =>
                                      pong_sums.contains(&Tile::Character(u.clone()))
                                          && pong_sums.contains(&Tile::Circle(u.clone())),
                                  _ => unreachable!()
                              }) { Some(Han(2)) } else { None }
                          }), None);
        let honor_tiles =
            HandYaku::new("役牌 / Honor tiles", None,
                          Box::new(move |candidate: &Wait| {
                              let mut han = Han(0);
                              han += Han(candidate.node().open_sets.iter().filter(|set| {
                                  set.count(&Dragon::White.tile()) >= 3 || set.count(&Dragon::Green.tile()) >= 3 || set.count(&Dragon::Red.tile()) >= 3 ||
                                      if let Some(seat_wind) = seat_wind.clone() {
                                          set.count(&seat_wind) >= 3
                                      } else { false } ||
                                      if let Some(wind) = prevalent_wind.clone() {
                                          set.count(&wind) >= 3
                                      } else { false }
                              }).count() as u32);
                              han += Han(candidate.node().sets.iter().filter(|set| {
                                  set.count(&Dragon::White.tile()) >= 3 || set.count(&Dragon::Green.tile()) >= 3 || set.count(&Dragon::Red.tile()) >= 3 ||
                                      if let Some(seat_wind) = seat_wind.clone() {
                                          set.count(&seat_wind) >= 3
                                      } else { false } ||
                                      if let Some(wind) = prevalent_wind.clone() {
                                          set.count(&wind) >= 3
                                      } else { false }
                              }).count() as u32);

                              if han == Han(0) {
                                  None
                              } else { Some(han) }
                          }), None);
        let terminal_or_honor_in_each_set =
            HandYaku::new("混全帯么九 / Terminal or honor in each set", None,
                          Box::new(|candidate: &Wait| {
                              if candidate.node().sets.iter().all(|set| { set.contains_yaotyu() })
                                  && candidate.node().open_sets.iter().all(|set| set.contains_yaotyu()) {
                                  if candidate.closed() {
                                      Some(Han(2))
                                  } else { Some(Han(1)) }
                              } else { None }
                          }), None);
        let terminal_in_each_set =
            HandYaku::new("純全帯么九 / Terminal in each set", Some(Box::new(terminal_or_honor_in_each_set)),
                          Box::new(|candidate: &Wait| {
                              if candidate.node().sets.iter().all(|set| set.contains_terminal())
                                  && candidate.node().open_sets.iter().all(|set| set.contains_terminal()) {
                                  if candidate.closed() {
                                      Some(Han(3))
                                  } else { Some(Han(2)) }
                              } else { None }
                          }), None);
        let all_terminals_and_honors =
            HandYaku::new("混老頭 / All terminals and honors", Some(Box::new(terminal_in_each_set)),
                          Box::new(|candidate: &Wait| {
                              if candidate.node().sets.iter().all(|set| set.contains_yaotyu() && set.is_flat())
                                  && candidate.node().open_sets.iter().all(|set| set.contains_yaotyu() && set.is_flat()) {
                                  if candidate.closed() {
                                      Some(Han(2))
                                  } else { Some(Han(2)) }
                              } else { None }
                          }), None);
        let little_three_dragons =
            HandYaku::new("小三元 / Little three dragons", None,
                          Box::new(|candidate: &Wait| {
                              if candidate.node().sets.iter().any(|set| set.count(&Dragon::White.tile()) >= 3)
                                  || candidate.node().open_sets.iter().any(|set| set.count(&Dragon::White.tile()) >= 3)
                                  && candidate.node().sets.iter().any(|set| set.count(&Dragon::Green.tile()) >= 3)
                                  || candidate.node().open_sets.iter().any(|set| set.count(&Dragon::Green.tile()) >= 3)
                                  && candidate.node().sets.iter().any(|set| set.count(&Dragon::Red.tile()) >= 3)
                                  || candidate.node().open_sets.iter().any(|set| set.count(&Dragon::Red.tile()) >= 3)
                              {
                                  Some(Han(2))
                              } else { None }
                          }), None);
        let half_flush =
            HandYaku::new("混一色 / Half flush", None,
                          Box::new(|candidate: &Wait| {
                              let node = candidate.node();
                              if node.sets.iter().all(|set| set.all_character() || set.all_honor())
                                  && node.open_sets.iter().all(|set| set.all_circle() || set.all_honor())
                                  || node.sets.iter().all(|set| set.all_character() || set.all_honor())
                                  && node.open_sets.iter().all(|set| set.all_circle() || set.all_honor())
                                  || node.sets.iter().all(|set| set.all_bamboo() || set.all_honor())
                                  && node.open_sets.iter().all(|set| set.all_bamboo() || set.all_honor())
                              {
                                  if candidate.closed() {
                                      Some(Han(3))
                                  } else { Some(Han(2)) }
                              } else { None }
                          }), None);
        let flush =
            HandYaku::new("清一色 / Flush", Some(Box::new(half_flush)),
                          Box::new(|candidate: &Wait| {
                              let node = candidate.node();
                              if node.sets.iter().all(|set| set.all_character())
                                  && node.open_sets.iter().all(|set| set.all_circle())
                                  || node.sets.iter().all(|set| set.all_character())
                                  && node.open_sets.iter().all(|set| set.all_circle())
                                  || node.sets.iter().all(|set| set.all_bamboo())
                                  && node.open_sets.iter().all(|set| set.all_bamboo())
                              {
                                  if candidate.closed() {
                                      Some(Han(6))
                                  } else { Some(Han(5)) }
                              } else { None }
                          }), None);


        vec![all_simple, nopoints, three_closed_triplets, three_colour_triplets, little_three_dragons, all_terminals_and_honors, all_triplet_hand, flush, straight, twoset, three_colour_straight, seven_pairs, honor_tiles]
    }
    pub fn default_adopted_yakuman_list() -> Vec<Yakuman> {
        let thirteen_orphans =
            Yakuman::new("国士無双 / Thirteen orphans", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                if _original_tiles.contains(&Tile::Character(1))
                    && _original_tiles.contains(&Tile::Character(9))
                    && _original_tiles.contains(&Tile::Circle(1))
                    && _original_tiles.contains(&Tile::Circle(9))
                    && _original_tiles.contains(&Tile::Bamboo(1))
                    && _original_tiles.contains(&Tile::Bamboo(9))
                    && _original_tiles.contains(&Wind::East.tile())
                    && _original_tiles.contains(&Wind::South.tile())
                    && _original_tiles.contains(&Wind::West.tile())
                    && _original_tiles.contains(&Wind::North.tile())
                    && _original_tiles.contains(&Dragon::White.tile())
                    && _original_tiles.contains(&Dragon::Green.tile())
                    && _original_tiles.contains(&Dragon::Red.tile())
                    && _original_tiles.all_yaotyu() { 1 } else { 0 }
            }), None);
        let thirteen_orphans_13_wait =
            Yakuman::new("国士無双一三面待ち / Thirteen orphans 13 wait", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                if _original_tiles.len() != 14 { return 0; }
                let winning = _candidate.winning();
                let mut original_tiles: Vec<Tile> = _original_tiles.iter().filter(|tile| tile != &&winning).map(|t| t.clone()).collect();
                original_tiles.push(winning);
                original_tiles.sort();
                if original_tiles.len() != 13 { return 0; }
                if original_tiles[..13] ==
                    vec![Tile::Character(1),
                         Tile::Character(9),
                         Tile::Circle(1),
                         Tile::Circle(9),
                         Tile::Bamboo(1),
                         Tile::Bamboo(9),
                         Wind::East.tile(),
                         Wind::South.tile(),
                         Wind::West.tile(),
                         Wind::North.tile(),
                         Dragon::White.tile(),
                         Dragon::Green.tile(),
                         Dragon::Red.tile()][..]
                    && original_tiles.all_yaotyu() { 2 } else { 0 }
            }), Some(Box::new(thirteen_orphans)));
        let big_three_dragons =
            Yakuman::new("大三元 / Big three dragons", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                let dragons = vec![Dragon::White.tile(), Dragon::Green.tile(), Dragon::Red.tile()];
                if dragons.iter().all(|dragon| {
                    _candidate.node().sets.iter().any(|set| set.count(dragon) >= 3)
                        || _candidate.node().open_sets.iter().any(|set| set.count(dragon) >= 3)
                }) { 1 } else { 0 }
            }), None);
        let four_concealed_triplets =
            Yakuman::new("四暗刻 / Four concealed triplets", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                if _candidate.node().sets.iter().filter(|set| set.is_flat()).count() +
                    _candidate.node().open_sets
                        .iter()
                        .filter(|set| match set {
                            OpenSet::ConcealedKong(_) => true,
                            _ => false,
                        }).count() == 4 + 1
                { 1 } else { 0 }
            }), None);
        let little_four_winds =
            Yakuman::new("小四喜 / Little four dragons", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                let dragons = vec![Wind::East.tile(), Wind::South.tile(), Wind::West.tile(), Wind::North.tile()];
                if dragons.iter().all(|dragon| {
                    _candidate.node().sets.iter().any(|set| set.count(dragon) >= 2)
                        || _candidate.node().open_sets.iter().any(|set| set.count(dragon) >= 2)
                }) { 1 } else { 0 }
            }), None);
        let big_four_winds =
            Yakuman::new("大四喜 / Big four dragons", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                let dragons = vec![Wind::East.tile(), Wind::South.tile(), Wind::West.tile(), Wind::North.tile()];
                if dragons.iter().all(|dragon| {
                    _candidate.node().sets.iter().any(|set| set.count(dragon) >= 3)
                        || _candidate.node().open_sets.iter().any(|set| set.count(dragon) >= 3)
                }) { 1 } else { 0 }
            }), Some(Box::new(little_four_winds)));
        let all_honors =
            Yakuman::new("字一色 / All honors", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                if _candidate.node().sets.iter().all(|set| set.all_honor())
                    && _candidate.node().open_sets.iter().all(|set| set.all_honor()) {
                    1
                } else { 0 }
            }), None);
        let all_terminals =
            Yakuman::new("清老頭 / All terminals", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                if _candidate.node().sets.iter().all(|set| set.all_terminal())
                    && _candidate.node().open_sets.iter().all(|set| set.all_terminal()) {
                    1
                } else { 0 }
            }), None);
        let all_green =
            Yakuman::new("緑一色 / All green", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                let greens = vec![Tile::Bamboo(2), Tile::Bamboo(3), Tile::Bamboo(4), Tile::Bamboo(6), Tile::Bamboo(8), Dragon::Green.tile()];
                if _candidate.node().sets.iter().all(|set| set.consists_of(&greens))
                    && _candidate.node().open_sets.iter().all(|set| set.consists_of(&greens))
                { 1 } else { 0 }
            }), None);
        let nine_gates =
            Yakuman::new("九蓮宝燈 / Nine gates", Box::new(|_candidate: &Wait, _original_tiles: &Vec<Tile>| {
                let mut original_tiles = _original_tiles.clone();
                original_tiles.sort();
                if original_tiles.count(&Tile::Character(1)) >= 3
                    && original_tiles.count(&Tile::Character(2)) >= 1
                    && original_tiles.count(&Tile::Character(3)) >= 1
                    && original_tiles.count(&Tile::Character(4)) >= 1
                    && original_tiles.count(&Tile::Character(5)) >= 1
                    && original_tiles.count(&Tile::Character(6)) >= 1
                    && original_tiles.count(&Tile::Character(7)) >= 1
                    && original_tiles.count(&Tile::Character(8)) >= 1
                    && original_tiles.count(&Tile::Character(9)) >= 3
                    && _candidate.node().sets.iter().all(|set| set.all_character())
                    && _candidate.node().open_sets.iter().all(|set| set.all_character())
                    ||
                    original_tiles.count(&Tile::Circle(1)) >= 3
                        && original_tiles.count(&Tile::Circle(2)) >= 1
                        && original_tiles.count(&Tile::Circle(3)) >= 1
                        && original_tiles.count(&Tile::Circle(4)) >= 1
                        && original_tiles.count(&Tile::Circle(5)) >= 1
                        && original_tiles.count(&Tile::Circle(6)) >= 1
                        && original_tiles.count(&Tile::Circle(7)) >= 1
                        && original_tiles.count(&Tile::Circle(8)) >= 1
                        && original_tiles.count(&Tile::Circle(9)) >= 3
                        && _candidate.node().sets.iter().all(|set| set.all_circle())
                        && _candidate.node().open_sets.iter().all(|set| set.all_circle())
                    ||
                    original_tiles.count(&Tile::Bamboo(1)) >= 3
                        && original_tiles.count(&Tile::Bamboo(2)) >= 1
                        && original_tiles.count(&Tile::Bamboo(3)) >= 1
                        && original_tiles.count(&Tile::Bamboo(4)) >= 1
                        && original_tiles.count(&Tile::Bamboo(5)) >= 1
                        && original_tiles.count(&Tile::Bamboo(6)) >= 1
                        && original_tiles.count(&Tile::Bamboo(7)) >= 1
                        && original_tiles.count(&Tile::Bamboo(8)) >= 1
                        && original_tiles.count(&Tile::Bamboo(9)) >= 3
                        && _candidate.node().sets.iter().all(|set| set.all_bamboo())
                        && _candidate.node().open_sets.iter().all(|set| set.all_bamboo())
                { 1 } else { 0 }
            }), None);

        vec![thirteen_orphans_13_wait, big_three_dragons, four_concealed_triplets, big_four_winds, all_honors, all_terminals, nine_gates, all_green]
    }
}

impl Evaluator {
    pub fn evaluate(&self, parsed_hand: &ParsedHand, draw: bool, situation: &Vec<SituationYaku>) -> Option<Evaluated> {
        self.evaluate_all(parsed_hand, draw, situation).last().cloned()
    }

    pub fn evaluate_all(&self, parsed_hand: &ParsedHand, draw: bool, situation: &Vec<SituationYaku>) -> Vec<Evaluated> {
        let waits = Waits::from_vec(parsed_hand, draw, &self.prevalent_wind, &self.seat_wind);
        let mut scores: Vec<Evaluated> = waits.waits
            .iter().map(|c| self.evaluate_wait(&waits.original_hand, c, draw, situation)).collect();
        scores.sort_by(|a, b| a.score.score(false).cmp(&b.score.score(false)));
        scores
    }

    fn evaluate_yaku(&self, yaku: &Box<HandYaku>, yaku_list: &mut Vec<String>, candidate: &Wait, han: &mut Han) {
        let rule = &yaku.rule;
        if let Some(han_) = rule(candidate) {
            *han += han_;
            yaku_list.push(yaku.name());
        } else {
            if let Some(ref yaku) = yaku.sub {
                self.evaluate_yaku(yaku, yaku_list, candidate, han);
            }
        }
    }

    fn evaluate_yakuman(&self, yakuman: &Box<Yakuman>, yakuman_list: &mut Vec<String>, candidate: &Wait, original_hand: &Vec<Tile>) -> u32 {
        let rule = &yakuman.rule;
        if 0 != rule(candidate, &original_hand) {
            yakuman_list.push(yakuman.name());
            rule(candidate, &original_hand)
        } else {
            if let Some(ref yakuman) = yakuman.sub {
                self.evaluate_yakuman(yakuman, yakuman_list, candidate, original_hand)
            } else { 0 }
        }
    }

    fn evaluate_wait(&self, original_hand: &Vec<Tile>, wait: &Wait, draw: bool, situation: &Vec<SituationYaku>) -> Evaluated {
        let mut yakuman_list = Vec::new();
        let mut multiple = 0;
        self.adopted_yakuman_list.iter().for_each(|yakuman| {
            let rule = &yakuman.rule;
            if 0 != rule(wait, &original_hand) {
                yakuman_list.push(yakuman.name());
                multiple += rule(wait, &original_hand);
            } else {
                if let Some(ref yakuman) = yakuman.sub {
                    multiple += self.evaluate_yakuman(yakuman, &mut yakuman_list, wait, original_hand);
                }
            }
        });
        if multiple != 0 {
            return Evaluated { score: Score::yakuman(multiple as u8), node: wait.node().clone(), yaku_list: yakuman_list };
        }

        let mut han = Han(0);
        let mut fu = Option::None;
        let mut yaku_list = Vec::new();
        for st in situation {
            yaku_list.push(st.name());
            han += st.han_value();
        }
        for adopted_yaku in &self.adopted_yaku_list {
            let rule = &adopted_yaku.rule;
            if let Some(han_) = rule(wait) {
                han += han_;
                yaku_list.push(adopted_yaku.name());
                // 平和等の場合
                if let Some(fu_rule) = &adopted_yaku.fu {
                    fu = Some(fu_rule(&draw));
                }
            } else {
                if let Some(ref yaku) = adopted_yaku.sub {
                    self.evaluate_yaku(yaku, &mut yaku_list, wait, &mut han);
                }
            }
        }


        let fu = match fu {
            Some(fu) => fu,
            None => wait.fu(),
        };

        Evaluated { score: Score::new(han, fu), node: wait.node().clone(), yaku_list }
    }

    pub fn evaluate_str(&self, string: &str, draw: bool, situation: &Vec<SituationYaku>) -> Result<Option<Evaluated>, failure::Error> {
        let hand = Hand::from_str(string)?;
        let parsed_hand = ParsedHand::new(&hand);
        Ok(Evaluator::new(None, None, Vec::new(), Vec::new()).evaluate(&parsed_hand, draw, situation))
    }

    pub fn evaluate_all_str(&self, string: &str, draw: bool, situation: &Vec<SituationYaku>) -> Result<Vec<Evaluated>, failure::Error> {
        let hand = Hand::from_str(string)?;
        let parsed_hand = ParsedHand::new(&hand);
        Ok(Evaluator::new(None, None, Vec::new(), Vec::new()).evaluate_all(&parsed_hand, draw, situation))
    }
}

/// 牌形と点数
#[derive(Clone)]
pub struct Evaluated {
    node: Node,
    score: Score,
    yaku_list: Vec<String>,
}

impl Display for Evaluated {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "{}", self.node)?;
        writeln!(f, "{}", self.yaku_list.join(","))?;
        writeln!(f, "{}", self.score)
    }
}

/// 最終型候補一覧
#[derive(Debug)]
pub struct Waits {
    original_hand: Vec<Tile>,
    waits: Vec<Wait>,
}

impl Waits {
    pub fn new(original_hand: Vec<Tile>, waits: Vec<Wait>) -> Self {
        Waits { original_hand, waits }
    }

    pub fn from_vec(parsed_hand: &ParsedHand, draw: bool, prevalent_wind: &Option<Tile>, seat_wind: &Option<Tile>) -> Waits {
        let nodes = parsed_hand.nodes.clone();
        let mut waits = Vec::new();
        nodes.iter().for_each(|node|
            waits.append(&mut Wait::from(&node, parsed_hand.winning.clone(),
                                         draw, prevalent_wind.clone(),
                                         seat_wind.clone())));
        let original_hand = parsed_hand.tiles.clone();
        Waits::new(original_hand, waits)
    }
}

/// 最終形候補
#[derive(Debug)]
pub enum Wait {
    Ryanmen(Node, Fu, Tile),
    Kanchan(Node, Fu, Tile),
    Penchan(Node, Fu, Tile),
    Tanki(Node, Fu, Tile),
    Shanpon(Node, Fu, Tile),
}

impl Wait {
    fn from(node: &Node, winning: Tile, draw: bool, prevalent_wind: Option<Tile>, seat_wind: Option<Tile>) -> Vec<Wait> {
        // 待ち候補
        let mut wait_candidates = Vec::new();
        node.sets.iter().for_each(|set| {
            if set.count(&winning) > 0 { wait_candidates.push(set.clone()) }
        });
        let mut candidates = Vec::new();

        // 府計算
        let mut fu = if draw { Fu(22) } else if node.open_sets.len() == 0 { Fu(30) } else { Fu(20) };
        node.sets.iter().for_each(|set| fu += set.fu());
        node.open_sets.iter().for_each(|set| fu += set.fu());

        // 雀頭による符
        if let Some(Set::Pair(head)) = node.sets.first() {
            let (head1, head2) = (head.get(0), head.get(1));
            if (head1, head2) == (prevalent_wind.as_ref(), prevalent_wind.as_ref())
                || (head1, head2) == (seat_wind.as_ref(), seat_wind.as_ref())
                || (head1, head2) == (Some(&Dragon::White.tile()), Some(&Dragon::White.tile()))
                || (head1, head2) == (Some(&Dragon::Green.tile()), Some(&Dragon::Green.tile()))
                || (head1, head2) == (Some(&Dragon::Red.tile()), Some(&Dragon::Red.tile())) {
                fu += Fu(2);
            }
        }

        wait_candidates.iter().for_each(|set| {
            match set {
                Set::Pair(_) => {
                    candidates.push(Wait::Tanki(node.clone(), fu + Fu(2), winning.clone()));
                }
                _ => {
                    if set.is_flat() {
                        candidates.push(Wait::Shanpon(node.clone(), fu, winning.clone()));
                    } else {
                        let set = match set.clone() {
                            Set::Chow(set) => set,
                            _ => unreachable!()
                        };
                        if Some(&winning) == set.first() || Some(&winning) == set.last() {
                            if set.contains_yaotyu() && !winning.is_yaotyu() {
                                candidates.push(Wait::Penchan(node.clone(), fu + Fu(2), winning.clone()));
                            } else {
                                candidates.push(Wait::Ryanmen(node.clone(), fu, winning.clone()));
                            }
                        } else {
                            candidates.push(Wait::Kanchan(node.clone(), fu + Fu(2), winning.clone()));
                        }
                    }
                }
            }
        });

        candidates
    }

    fn winning(&self) -> Tile {
        match &self {
            Wait::Ryanmen(_, _, winning) => winning,
            Wait::Kanchan(_, _, winning) => winning,
            Wait::Penchan(_, _, winning) => winning,
            Wait::Tanki(_, _, winning) => winning,
            Wait::Shanpon(_, _, winning) => winning,
        }.clone()
    }

    /// 門前orNot
    pub fn closed(&self) -> bool {
        match &self {
            Wait::Ryanmen(node, _, _) => {
                node
            }
            Wait::Kanchan(node, _, _) => {
                node
            }
            Wait::Penchan(node, _, _) => {
                node
            }
            Wait::Shanpon(node, _, _) => {
                node
            }
            Wait::Tanki(node, _, _) => {
                node
            }
        }.open_sets.iter().all(|set| match set {
            OpenSet::ConcealedKong(_) => true,
            _ => false,
        })
    }

    /// nodeを取得
    pub fn node(&self) -> &Node {
        match &self {
            Wait::Ryanmen(node, _, _) => {
                node
            }
            Wait::Kanchan(node, _, _) => {
                node
            }
            Wait::Penchan(node, _, _) => {
                node
            }
            Wait::Shanpon(node, _, _) => {
                node
            }
            Wait::Tanki(node, _, _) => {
                node
            }
        }
    }

    /// 府数を取得
    pub fn fu(&self) -> Fu {
        match &self {
            Wait::Ryanmen(_, fu, _) => fu,
            Wait::Kanchan(_, fu, _) => fu,
            Wait::Penchan(_, fu, _) => fu,
            Wait::Tanki(_, fu, _) => fu,
            Wait::Shanpon(_, fu, _) => fu,
        }.clone()
    }
}

impl Display for Wait {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Wait::Ryanmen(node, fu, _) => {
                writeln!(f, "両面待ち: Ryanmen {} / {}", fu, node)
            }
            Wait::Kanchan(node, fu, _) => {
                writeln!(f, "嵌張待ち: Kanchan {} / {}", fu, node)
            }
            Wait::Penchan(node, fu, _) => {
                writeln!(f, "辺張待ち: Penchan {} / {}", fu, node)
            }
            Wait::Shanpon(node, fu, _) => {
                writeln!(f, "双碰待ち: Shanpon {} / {}", fu, node)
            }
            Wait::Tanki(node, fu, _) => {
                writeln!(f, "単騎待ち: Tanki {} / {}", fu, node)
            }
        }
    }
}
