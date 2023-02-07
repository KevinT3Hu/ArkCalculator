use std::collections::HashMap;

pub type SkillUpgradeCost = HashMap<String,u32>;

pub type SkillCost = [SkillUpgradeCost;3];

pub type Skills = Vec<SkillCost>;

pub fn parse_skills(skills:&serde_json::Value) -> Option<Skills>{
    let skills = skills.as_array();
    if skills.is_none(){
        return None;
    }
    let skills = skills.unwrap();
    let result = skills.iter().map(|skill|{
        let skill = skill.as_array().unwrap();
        let mut levels = [HashMap::new(),HashMap::new(),HashMap::new()];
        for index in 0..3{
            let cost = &skill[index].as_array().unwrap();
            for cost in *cost{
                let cost = cost.as_object().unwrap();
                let id = cost.get("id").unwrap().as_str().unwrap().to_string();
                let count = cost.get("count").unwrap().as_u64().unwrap() as u32;
                levels[index].insert(id,count);
            }
        }
        levels
    }).collect::<Vec<_>>();
    Some(result)
}