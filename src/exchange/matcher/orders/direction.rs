use diesel::pg::Pg;
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use std::fmt;
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromSqlRow, AsExpression, SqlType)]
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

impl serialize::ToSql<Text, Pg> for Direction {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        write!(out, "{}", self)
            .map(|_| serialize::IsNull::No)
            .map_err(Into::into)
    }
}

impl deserialize::FromSql<Text, Pg> for Direction {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match <String as deserialize::FromSql<Text, Pg>>::from_sql(bytes)
            .unwrap()
            .as_ref()
        {
            "Buy" => Ok(Direction::Buy),
            "Sell" => Ok(Direction::Sell),
            _ => Err(Into::into("Bad direction")),
        }
    }
}
