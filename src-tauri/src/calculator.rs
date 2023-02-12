use std::collections::HashMap;

use serde_json::Value;

use crate::{AppState, calculator_plugins::{OperatorTarget, OperatorInfo, calculate_cost_internal, calculate_skill_cost_internal, combine_cost, Stage, self}};

#[tauri::command]
pub fn get_operator_list(app_state: tauri::State<AppState>) -> Vec<OperatorInfo> {
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
pub fn calculate_cost(app_state: tauri::State<AppState>, target:OperatorTarget) -> HashMap<String, u32> {
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    let level_cost = calculate_cost_internal(&resource_loader, &target);
    let skills_cost = calculate_skill_cost_internal(&resource_loader, &target.name, &target.skill_targets);
    combine_cost(&level_cost, &skills_cost)
}

#[tauri::command]
pub fn calculate_total_cost(app_state: tauri::State<AppState>, targets:Vec<OperatorTarget>,use_lv:bool,use_skill:bool) -> HashMap<String, u32> {
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
pub fn get_material_name(app_state: tauri::State<AppState>, material_id:&str) -> String {
    let resource_loader = app_state.resource_loader.lock().unwrap();
    let resource_loader = resource_loader.as_ref().unwrap();
    resource_loader.get_material_name(material_id).unwrap()
}

// passing the param as a string and deserializing it is a workaround for the bug that HashMao would be empty when passed as a param
// see https://github.com/tauri-apps/tauri/issues/6078
#[tauri::command]
pub fn get_planner_plan(required:&str) -> Result<Vec<Stage>,String>{
    println!("required:{}",required);
    let required:Value = serde_json::from_str(required).unwrap();
    let required = required.as_array().unwrap();
    let required = required.iter().map(|value|{
        let value = value.as_array().unwrap();
        (value[0].as_str().unwrap().to_owned(),value[1].as_u64().unwrap() as u32)
    }).collect::<HashMap<String,u32>>();
    calculator_plugins::get_planner_plan(&required).map_err(|e|{e.to_string()})
}