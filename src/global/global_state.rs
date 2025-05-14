use std::sync::{RwLock};
use once_cell::sync::Lazy;

use crate::model::GeneratedResponse;


pub static GENERATED_DATA: Lazy<RwLock<Option<GeneratedResponse>>> = Lazy::new(|| RwLock::new(None));