use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Mood<'a>{
    pub activity: String,
    pub rating: i32,
}
