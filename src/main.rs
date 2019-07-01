extern crate mahjong;
use mahjong::calculator;

fn main() -> Result<(), failure::Error> {
    let scores = calculator::calc("23344567s2p3p4p8m8m2s", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("112s3344556677p2s", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("234m234p234s[234s]8p8p", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("123456778899s東東", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("111222333s[444s]55s", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("111s111m111p[444s]55s", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("111s999s111p999p中中", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("白白白発発発中中中[西西西]東東", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("[123s]567788889s東東", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("19m19p19s東南西北白発中1m", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("111s白白白発発発中中中11p", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("東東東南南南西西西北北北発発", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("東東東南南南西西西北北[^発発発発]", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("111999m111999p11s", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("223344s666s888s発発", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }
    let scores = calculator::calc("11112345678999s", Vec::new(), false)?;
    if let Some(score) = scores.last() {
        println!("{}", score);
    }

    Ok(())
}