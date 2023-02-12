use std::{fs, io::{Read, Write}};

use crate::{AppState, calculator_plugins::OperatorTarget};

pub struct ProfileStore{
    pub profile_directory_path: String,
    pub profile_list: Vec<String>
}

impl ProfileStore{
    pub fn new(profile_directory_path: &str) -> Self{
        if !std::path::Path::new(profile_directory_path).exists() {
            fs::create_dir(profile_directory_path).unwrap();
        }
        let profile_list = fs::read_dir(profile_directory_path).unwrap().map(|entry| entry.unwrap().file_name().into_string().unwrap().rsplit_once('.').unwrap().0.to_string()).collect();
        let mut profile_store = Self{
            profile_directory_path: profile_directory_path.to_owned(),
            profile_list
        };
        if profile_store.profile_list.is_empty() {
            profile_store.create_profile("Default");
        }
        profile_store
    }

    pub fn create_profile(&mut self, profile_name: &str){
        let profile_path = format!("{}/{}.json", self.profile_directory_path, profile_name);
        if !std::path::Path::new(&profile_path).exists() {
            fs::write(profile_name, "[]").unwrap();
        }
        self.profile_list.push(profile_name.to_owned());
    }

    pub fn delete_profile(&mut self, profile_name: &str){
        let profile_path = format!("{}/{}.json", self.profile_directory_path, profile_name);
        if std::path::Path::new(&profile_path).exists() {
            fs::remove_dir_all(profile_path).unwrap();
        }
        self.profile_list.retain(|name| name != profile_name);
    }

    pub fn load_profile(&self, profile_name: &str) -> Vec<OperatorTarget>{
        let profile_path = format!("{}/{}.json", self.profile_directory_path, profile_name);
        let mut profile = Vec::new();
        if std::path::Path::new(&profile_path).exists() {
            println!("Profile name: {}", profile_name);
            let mut profile_file = fs::File::open(profile_path).unwrap();
            let mut profile_json = String::new();
            profile_file.read_to_string(&mut profile_json).unwrap();
            profile = serde_json::from_str(&profile_json).unwrap();
        }
        profile
    }

    pub fn save_profile(&self, profile_name: &str, profile: &Vec<OperatorTarget>){
        let profile_path = format!("{}/{}.json", self.profile_directory_path, profile_name);
        let profile_json = serde_json::to_string(profile).unwrap();
        let mut profile_file = fs::File::create(profile_path).unwrap();
        profile_file.write_all(profile_json.as_bytes()).unwrap();
    }
}

#[tauri::command]
pub fn get_profile_list(app_state:tauri::State<AppState>) -> Vec<String>{
    let profile_store = app_state.profile_store.lock().unwrap();
    let profile_store = profile_store.as_ref().unwrap();
    profile_store.profile_list.clone()
}

#[tauri::command]
pub fn create_profile(app_state:tauri::State<AppState>, profile_name: String){
    let mut profile_store = app_state.profile_store.lock().unwrap();
    let profile_store = profile_store.as_mut().unwrap();
    profile_store.create_profile(&profile_name);
}

#[tauri::command]
pub fn delete_profile(app_state:tauri::State<AppState>, profile_name: String){
    let mut profile_store = app_state.profile_store.lock().unwrap();
    let profile_store = profile_store.as_mut().unwrap();
    profile_store.delete_profile(&profile_name);
}

#[tauri::command]
pub fn load_profile(app_state:tauri::State<AppState>, profile_name: String) -> Vec<OperatorTarget>{
    let profile_store = app_state.profile_store.lock().unwrap();
    let profile_store = profile_store.as_ref().unwrap();
    profile_store.load_profile(&profile_name)
}

#[tauri::command]
pub fn save_profile(app_state:tauri::State<AppState>, profile_name: String, profile: Vec<OperatorTarget>){
    let profile_store = app_state.profile_store.lock().unwrap();
    let profile_store = profile_store.as_ref().unwrap();
    profile_store.save_profile(&profile_name, &profile);
}