use std::collections::HashMap;

use ark_calculator::resource_loader::ResourceLoader;

#[derive(serde::Serialize)]
pub struct OperatorInfo{
    pub name:String,
    pub rarity:u8,
    pub skill_count:u8
}

#[derive(serde::Deserialize,Debug)]
pub struct OperatorTarget{
    pub name:String,
    pub current_elite:u8,
    pub target_elite:u8,
    pub current_level:u8,
    pub target_level:u8,
    pub skill_targets:Vec<SkillTarget>
}

#[derive(serde::Deserialize,Debug)]
pub struct SkillTarget{
    pub current_elite:u8,
    pub target_elite:u8,
}

pub fn calculate_cost_internal(loader:&ResourceLoader, target:&OperatorTarget) -> HashMap<String, u32> {
    let operator = loader.get_operator_list().unwrap().get(&target.name).unwrap();
    operator.calculate_cost(target.current_elite, target.current_level, target.target_elite, target.target_level,loader.get_upgrade_store().unwrap()).unwrap()
}

pub fn calculate_skill_cost_internal(loader:&ResourceLoader,operator_name:&str,skill_targets:&Vec<SkillTarget>)->HashMap<String,u32>{
    let operator = loader.get_operator_list().unwrap().get(operator_name).unwrap();
    let mut cost = HashMap::new();
    for (index,skill_target) in skill_targets.iter().enumerate(){
        let skill_cost = operator.calculate_skill_cost(index as u8, skill_target.current_elite, skill_target.target_elite).unwrap();
        cost = combine_cost(&cost,&skill_cost);
    }
    cost
}

pub fn combine_cost(a:&HashMap<String,u32>,b:&HashMap<String,u32>)->HashMap<String,u32>{
    let mut total_cost = HashMap::new();
    for (key,value) in a{
        let total = total_cost.entry(key.clone()).or_insert(0);
        *total += value;
    }
    for (key,value) in b{
        let total = total_cost.entry(key.clone()).or_insert(0);
        *total += value;
    }
    total_cost
}