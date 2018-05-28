use diesel::sql_types::Text;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum Direction {
    Buy,
    Sell,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Direction::Buy => "Buy",
                Direction::Sell => "Sell",
            }
        )
    }
}
