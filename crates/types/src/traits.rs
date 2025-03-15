use serde::{Deserialize, Serialize};

pub trait SerdeType: Serialize + for<'a> Deserialize<'a> {
    fn se(&self) -> Vec<u8>;
    fn de(ser: Vec<u8>) -> Self;
}
