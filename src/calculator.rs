use crate::evaluate::{Evaluated, Candidates, Evaluator};
use crate::yaku::situation::SituationYaku;
use crate::groups::Hand;
use crate::parse::Parser;
use std::str::FromStr;

pub fn calc(tiles_str: &str, situations: Vec<SituationYaku>, draw: bool) -> Result<Vec<Evaluated>, failure::Error> {
    let hand = Hand::from_str(tiles_str)?;
    let parser = Parser::new(&hand);
    let candidates = Candidates::from_vec(parser.tiles, parser.nodes, hand.winning.clone(), draw, None, None);
    Ok(Evaluator::new(None, None, Vec::new(), Vec::new()).evaluate_all(candidates))
}