use std::fmt::{Debug, Display};
use std::str::FromStr;

use serde::{de::DeserializeOwned, ser::Serialize};

use super::errors::Error;

pub trait Response: Serialize + DeserializeOwned + Debug + Display + FromStr<Err = Error> {}
