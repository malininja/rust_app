use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum UnitOfMeasure {
    Piece,
    Kg,
    Litre,
    Metre,
}
