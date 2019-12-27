use std::fmt::{Display, Formatter, Error};


#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Color {
    None,
    Black,
    Brown,
    Maroon,
    White,
    Red,
    Green,
    DarkGreen,
    Blue,
    Orange,
    Grey,
    LightGrey,
    DarkGrey,
}

impl Display for Color {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let c = format!("{:?}", self).to_lowercase();
        f.write_str(&c)
    }
}