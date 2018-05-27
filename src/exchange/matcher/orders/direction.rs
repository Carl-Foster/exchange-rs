use diesel::pg::Pg;
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use serde_json;
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, SqlType, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum Direction {
    Buy,
    Sell,
}

impl serialize::ToSql<Text, Pg> for Direction {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        serde_json::to_writer(out, self)
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
