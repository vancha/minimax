use std::fmt;


//this is what occupies a position on the grid
#[derive(PartialEq, Clone)]
pub enum BoardValue {
    EMPTY,
    X, //maximizing
    O, //minimizing
}

//make sure boardvalue can be printed on screen
impl fmt::Debug for BoardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            BoardValue::EMPTY => {
                write!(f, "   ")
            }
            BoardValue::X => {
                write!(f, " X ")
            }
            BoardValue::O => {
                write!(f, " O ")
            }
        }
    }
}
