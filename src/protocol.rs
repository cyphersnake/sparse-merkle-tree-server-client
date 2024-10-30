use serde::Serialize;

use crate::{Data, Proof};

#[derive(Debug, Serialize)]
pub enum Request {
    UpdateLeaf { leaf_index: usize, new_data: Data },
}

#[derive(Debug, Serialize)]
pub enum Response {
    Updated { proof: Proof },
}

