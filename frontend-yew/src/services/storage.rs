use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

pub fn get_string(key: &str) -> Option<String> {
    LocalStorage::get(key).ok()
}

pub fn set_string(key: &str, value: &str) {
    let _ = LocalStorage::set(key, value);
}

pub fn get_json<T: DeserializeOwned>(key: &str) -> Option<T> {
    LocalStorage::get(key).ok()
}

pub fn set_json<T: Serialize>(key: &str, value: &T) {
    let _ = LocalStorage::set(key, value);
}
