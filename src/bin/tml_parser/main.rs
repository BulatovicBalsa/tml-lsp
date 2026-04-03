use rustemo::Parser;
use crate::tml::TmlParser;

mod tml;
mod tml_actions;

fn main() {
    let snippet = r#""#;

    let parser = TmlParser::new();
    println!("{:#?}", parser.parse(snippet));
}