pub mod error;
pub mod db;
pub mod api;
pub mod model;
pub mod utils;
pub mod config;


use std::{collections::{BTreeSet, HashMap}, sync::{Arc, Mutex}};

type SafeVec<T> = Arc<Mutex<Vec<T>>>;
type SafeMap<K, V> = Arc<Mutex<HashMap<K, V>>>;
type SafeBTreeSet<T> = Arc<Mutex<BTreeSet<T>>>;

type Uuid = String;

#[cfg(test)]
mod test;