use std::{collections::HashMap, fs};

use serde_json::Value;

use crate::AppState;

pub struct ConfigStore{
    pub config_file_path: String,
    pub config: HashMap<String,Value>
}

impl ConfigStore{
    pub fn new(config_file_path: &str) -> Self{
        if !std::path::Path::new(config_file_path).exists() {
            fs::write(config_file_path, "{}").unwrap();
        }
        let config = fs::read_to_string(config_file_path).unwrap();
        let config: HashMap<String,Value> = serde_json::from_str(&config).unwrap();
        Self{
            config_file_path: config_file_path.to_owned(),
            config
        }
    }

    fn get(&self, key: &str) -> Option<&Value>{
        self.config.get(key)
    }

    pub fn get_with_def(&self, key: &str, default: Value) -> Value{
        match self.get(key){
            Some(value) => value.clone(),
            None => default
        }
    }

    pub fn set(&mut self, key: &str, value: Value){
        self.config.insert(key.to_owned(), value);
    }

    pub fn save(&self){
        let config = serde_json::to_string_pretty(&self.config).unwrap();
        std::fs::write(&self.config_file_path, config).unwrap();
    }
}

#[tauri::command]
pub fn get_config(app_state:tauri::State<AppState>, key: String, default:Value) -> Value{
    let config_store = app_state.config_store.lock().unwrap();
    let config_store = config_store.as_ref().unwrap();
    config_store.get_with_def(&key, default)
}

#[tauri::command]
pub fn set_config(app_state:tauri::State<AppState>, key: String, value: Value){
    let mut config_store = app_state.config_store.lock().unwrap();
    let config_store = config_store.as_mut().unwrap();
    config_store.set(&key, value);
    config_store.save();
}