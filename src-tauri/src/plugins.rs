use std::collections::HashMap;

use ark_calculator::resource_loader::ResourceLoader;

#[derive(serde::Serialize)]
pub struct OperatorInfo{
    pub name:String,
    pub rarity:u8
}

#[derive(serde::Deserialize,Debug)]
pub struct OperatorTarget{
    pub name:String,
    pub current_elite:u8,
    pub target_elite:u8,
    pub current_level:u8,
    pub target_level:u8,
}

pub fn calculate_cost_internal(loader:&ResourceLoader, target:OperatorTarget) -> HashMap<String, u32> {
    let operator = loader.get_operator_list().unwrap().get(&target.name).unwrap();
    operator.calculate_cost(target.current_elite, target.current_level, target.target_elite, target.target_level,loader.get_upgrade_store().unwrap()).unwrap()
}