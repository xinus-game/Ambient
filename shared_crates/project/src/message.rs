use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{ComponentType, Identifier};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Message {
    pub description: Option<String>,
    pub fields: BTreeMap<Identifier, ComponentType>,
}
