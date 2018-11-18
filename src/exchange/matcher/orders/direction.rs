use rocket::http::RawStr;
use rocket::request::FromParam;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Direction {
    Buy,
    Sell,
}

impl<'r> FromParam<'r> for Direction {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        match param.as_ref() {
            "Buy" => Ok(Direction::Buy),
            "Sell" => Ok(Direction::Sell),
            _ => Err(param),
        }
    }
}
