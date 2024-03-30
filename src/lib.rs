pub mod error;
pub mod db;
pub mod api;
pub mod model;
pub mod utils;


use std::{collections::HashMap, sync::{Arc, Mutex}};

type SafeVec<T> = Arc<Mutex<Vec<T>>>;
type SafeMap<K, V> = Arc<Mutex<HashMap<K, V>>>;
type Uuid = String;