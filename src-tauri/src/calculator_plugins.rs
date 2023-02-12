use std::collections::HashMap;

use ark_calculator::resource_loader::ResourceLoader;
use serde::de::Error;

#[derive(serde::Serialize)]
pub struct OperatorInfo {
    pub name: String,
    pub rarity: u8,
    pub skill_count: u8,
}

#[derive(serde::Deserialize,serde::Serialize, Debug)]
pub struct OperatorTarget {
    pub name: String,
    pub rarity: u8,
    pub current_elite: u8,
    pub target_elite: u8,
    pub current_level: u8,
    pub target_level: u8,
    pub skill_targets: Vec<SkillTarget>,
}

#[derive(serde::Deserialize,serde::Serialize, Debug)]
pub struct SkillTarget {
    pub current_elite: u8,
    pub target_elite: u8,
}

pub fn calculate_cost_internal(
    loader: &ResourceLoader,
    target: &OperatorTarget,
) -> HashMap<String, u32> {
    let operator = loader
        .get_operator_list()
        .unwrap()
        .get(&target.name)
        .unwrap();
    operator
        .calculate_cost(
            target.current_elite,
            target.current_level,
            target.target_elite,
            target.target_level,
            loader.get_upgrade_store().unwrap(),
        )
        .unwrap()
}

pub fn calculate_skill_cost_internal(
    loader: &ResourceLoader,
    operator_name: &str,
    skill_targets: &Vec<SkillTarget>,
) -> HashMap<String, u32> {
    let operator = loader
        .get_operator_list()
        .unwrap()
        .get(operator_name)
        .unwrap();
    let mut cost = HashMap::new();
    for (index, skill_target) in skill_targets.iter().enumerate() {
        let skill_cost = operator
            .calculate_skill_cost(
                index as u8,
                skill_target.current_elite,
                skill_target.target_elite,
            )
            .unwrap();
        cost = combine_cost(&cost, &skill_cost);
    }
    cost
}

pub fn combine_cost(a: &HashMap<String, u32>, b: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut total_cost = HashMap::new();
    for (key, value) in a {
        let total = total_cost.entry(key.clone()).or_insert(0);
        *total += value;
    }
    for (key, value) in b {
        let total = total_cost.entry(key.clone()).or_insert(0);
        *total += value;
    }
    total_cost
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PlannerResponse {
    pub cost: u32,
    pub stages: Vec<Stage>,
}

#[derive(serde::Serialize, Debug)]
pub struct Stage {
    pub name: String,
    pub count: u32,
}

impl<'de> serde::Deserialize<'de> for Stage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let mut stage = Stage {
            name: String::new(),
            count: 0,
        };
        match value.as_object() {
            Some(value) => {
                for (key, value) in value {
                    match key.as_str() {
                        "stage" => match value.as_str() {
                            Some(value) => {
                                stage.name = value.to_string();
                            }
                            None => {
                                return Err(D::Error::custom("Stage name is not a string"));
                            }
                        },
                        "count" => match value.as_str() {
                            Some(value) => match value.parse::<f32>() {
                                Ok(value) => {
                                    stage.count = value.round() as u32;
                                }
                                Err(_) => {
                                    return Err(D::Error::custom(format!(
                                        "Stage count {} is not a number",
                                        value
                                    )));
                                }
                            },
                            None => {
                                return Err(D::Error::custom("Stage count is not a string"));
                            }
                        },
                        _ => {}
                    }
                }
            }
            None => {
                return Err(D::Error::custom("Stage is not an object"));
            }
        }
        Ok(stage)
    }
}

pub fn get_planner_plan(required: &HashMap<String, u32>) -> Result<Vec<Stage>, ureq::Error> {
    println!("{:?}", required);
    let url = "https://planner.penguin-stats.io/plan";
    let result: PlannerResponse = ureq::post(url)
        .send_json(ureq::json!({ "required": required }))?
        .into_json()?;
    Ok(result.stages)
}
