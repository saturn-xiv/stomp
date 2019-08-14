use std::fmt::{Debug, Display};

use serde::{de::DeserializeOwned, ser::Serialize};

pub trait Request: Serialize + DeserializeOwned + Debug + Display {}
