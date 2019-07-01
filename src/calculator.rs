use crate::evaluate::{Evaluated, Evaluator};
use crate::yaku::situation::SituationYaku;

pub fn calc(tiles_str: &str, situations: &Vec<SituationYaku>, draw: bool) -> Result<Vec<Evaluated>, failure::Error> {
    Evaluator::new(None, None, Vec::new(), Vec::new()).evaluate_all_str(tiles_str, draw, situations)
}