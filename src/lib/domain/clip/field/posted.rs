use crate::domain::Time;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
// use std::str::FromStr;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}
