#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugins;

use std::{collections::HashMap, sync::Mutex};

use ark_calculator::resource_loader::ResourceLoader;
use plugins::{calculate_skill_cost_internal, combine_cost};

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
        let skill_count = match &operator.skills {
            None => 0,
            Some(skills) => skills.len() as u8
        };
        OperatorInfo{
            name:operator.name.clone(),
            rarity:operator.rarity,
            skill_count
        }
    }).collect()
}

#[tauri::command]
fn calculate_cost(app_state: tauri::State<AppState>, target:OperatorTarget) -> HashMap<String, u32> {
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    let level_cost = calculate_cost_internal(&resource_loader, &target);
    let skills_cost = calculate_skill_cost_internal(&resource_loader, &target.name, &target.skill_targets);
    combine_cost(&level_cost, &skills_cost)
}

#[tauri::command]
fn calculate_total_cost(app_state: tauri::State<AppState>, targets:Vec<OperatorTarget>,use_lv:bool,use_skill:bool) -> HashMap<String, u32> {
    let mut total_cost = HashMap::new();
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    println!("{:?}",targets);
    for target in targets{
        let lv_cost = if use_lv {
            calculate_cost_internal(&resource_loader, &target)
        } else {
            HashMap::new()
        };
        let skills_cost = if use_skill {
            calculate_skill_cost_internal(&resource_loader, &target.name, &target.skill_targets)
        } else {
            HashMap::new()
        };
        let cost = combine_cost(&lv_cost, &skills_cost);
        total_cost = combine_cost(&total_cost, &cost);
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
