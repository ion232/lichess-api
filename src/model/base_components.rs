use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ok {
    pub ok: bool
}

impl Into<Result<bool>> for Ok {
    fn into(self) -> Result<bool> {
        Ok(self.ok)
    }
}
