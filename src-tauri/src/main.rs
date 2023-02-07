#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugins;

use std::{collections::HashMap, sync::Mutex};

use ark_calculator::resource_loader::ResourceLoader;

use crate::plugins::{calculate_cost_internal, OperatorInfo, OperatorTarget};

struct AppState{
    resource_loader:Mutex<Option<ResourceLoader>>
}

#[tauri::command]
fn init(app_handle:tauri::AppHandle,app_state: tauri::State<AppState>) -> Result<(),String> {
    let path = app_handle.path_resolver().resolve_resource("json").unwrap().to_str().unwrap().to_owned();
    let resource_loader = ResourceLoader::new(&path);
    *app_state.resource_loader.lock().unwrap() = Some(resource_loader);
    Ok(())
}

#[tauri::command]
fn get_operator_list(app_state: tauri::State<AppState>) -> Vec<OperatorInfo> {
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    resource_loader.get_operator_list().unwrap().iter().map(|(_,operator)|{
        OperatorInfo{
            name:operator.name.clone(),
            rarity:operator.rarity
        }
    }).collect()
}

#[tauri::command]
fn calculate_cost(app_state: tauri::State<AppState>, target:OperatorTarget) -> HashMap<String, u32> {
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    calculate_cost_internal(&resource_loader, target)
}

#[tauri::command]
fn calculate_total_cost(app_state: tauri::State<AppState>, targets:Vec<OperatorTarget>) -> HashMap<String, u32> {
    let mut total_cost = HashMap::new();
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    println!("{:?}",targets);
    for target in targets{
        let cost = calculate_cost_internal(&resource_loader, target);
        for (key,value) in cost{
            println!("{}:{}",key,value);
            let total = total_cost.entry(key).or_insert(0);
            *total += value;
        }
    }
    total_cost
}

#[tauri::command]
fn get_material_name(app_state: tauri::State<AppState>, material_id:&str) -> String {
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    resource_loader.get_material_name(material_id).unwrap()
}

fn main() {
    let app_state = AppState{
        resource_loader:Mutex::new(None)
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![get_operator_list,calculate_cost,calculate_total_cost,init,get_material_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
