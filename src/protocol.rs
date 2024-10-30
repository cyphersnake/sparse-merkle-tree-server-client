pub use bincode;
use serde::{Deserialize, Serialize};

use crate::{Data, Proof};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    UpdateLeaf { leaf_index: u32, new_data: Data },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Err { msg: String },
    Updated { proof: Box<Proof> },
}
