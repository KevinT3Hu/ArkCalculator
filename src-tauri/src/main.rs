#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod calculator;
mod calculator_plugins;
mod config;
mod profile;

use std::sync::Mutex;

use ark_calculator::resource_loader::ResourceLoader;
use config::ConfigStore;
use profile::ProfileStore;

pub struct AppState {
    resource_loader: Mutex<Option<ResourceLoader>>,
    config_store: Mutex<Option<ConfigStore>>,
    profile_store: Mutex<Option<ProfileStore>>,
}

#[tauri::command]
fn init(app_handle: tauri::AppHandle, app_state: tauri::State<AppState>) -> Result<(), String> {
    let json_resource_path = app_handle
        .path_resolver()
        .resolve_resource("json")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let resource_loader = ResourceLoader::new(&json_resource_path);
    *app_state.resource_loader.lock().unwrap() = Some(resource_loader);

    let path = app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap()
        .join("config.json");
    let config_file_path = path.to_str().unwrap();
    let config_store = ConfigStore::new(config_file_path);
    *app_state.config_store.lock().unwrap() = Some(config_store);

    let path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("profiles");
    let profile_directory_path = path.to_str().unwrap();
    let profile_store = ProfileStore::new(profile_directory_path);
    *app_state.profile_store.lock().unwrap() = Some(profile_store);

    Ok(())
}

fn main() {
    let app_state = AppState {
        resource_loader: Mutex::new(None),
        config_store: Mutex::new(None),
        profile_store: Mutex::new(None),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            init,
            calculator::get_operator_list,
            calculator::calculate_cost,
            calculator::calculate_total_cost,
            calculator::get_material_name,
            calculator::get_planner_plan,
            config::get_config,
            config::set_config,
            profile::get_profile_list,
            profile::create_profile,
            profile::delete_profile,
            profile::load_profile,
            profile::save_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
