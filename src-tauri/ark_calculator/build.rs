use std::fs::File;
use std::io::{Read, Write};

use serde_json::{json, Value};

fn main() {
    let resource_path = "../json";
    println!("cargo:rerun-if-changed=build.rs");
    //create resource dir if not exist
    if !std::path::Path::new(resource_path).exists() {
        std::fs::create_dir(resource_path).unwrap();
    }
    handle_upgrade_data_json(resource_path);
    handle_material_json(resource_path);
    handle_character_json(resource_path);
}

fn handle_upgrade_data_json(out_dir: &str) {
    let upgrade_data_json_path = "./ArknightsGameData/zh_CN/gamedata/excel/gamedata_const.json";
    println!("cargo:rerun-if-changed={}", upgrade_data_json_path);
    let mut file = File::open(upgrade_data_json_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let upgrade_data_json: Value = serde_json::from_str(&content).unwrap();
    let upgrade_data_json = upgrade_data_json.as_object().unwrap();

    //get exp cost
    let exp_cost = upgrade_data_json.get("characterExpMap").unwrap().as_array().unwrap();
    let exp_cost_0 = &exp_cost[0].as_array().unwrap()[..49];
    let exp_cost_1 = &exp_cost[1].as_array().unwrap()[..79];
    let exp_cost_2 = &exp_cost[2].as_array().unwrap()[..89];
    let exp_cost = vec![exp_cost_0, exp_cost_1, exp_cost_2];

    //get upgrade cost
    let upgrade_cost = upgrade_data_json.get("characterUpgradeCostMap").unwrap().as_array().unwrap();
    let upgrade_cost_0 = &upgrade_cost[0].as_array().unwrap()[..49];
    let upgrade_cost_1 = &upgrade_cost[1].as_array().unwrap()[..79];
    let upgrade_cost_2 = &upgrade_cost[2].as_array().unwrap()[..89];
    let upgrade_cost = vec![upgrade_cost_0, upgrade_cost_1, upgrade_cost_2];

    //get evolve cost
    let evolve_cost = upgrade_data_json.get("evolveGoldCost").unwrap().as_array().unwrap();
    let evolve_cost_2 = &evolve_cost[2].as_array().unwrap()[..1];
    let evolve_cost_3 = evolve_cost[3].as_array().unwrap();
    let evolve_cost_4 = evolve_cost[4].as_array().unwrap();
    let evolve_cost_5 = evolve_cost[5].as_array().unwrap();
    let evolve_cost = vec![evolve_cost_2, evolve_cost_3, evolve_cost_4, evolve_cost_5];

    //write to file
    let mut file = File::create(format!("{}/upgrade_data.json", out_dir)).unwrap();
    let upgrade_data_json = json!({
        "exp_cost":exp_cost,
        "upgrade_cost":upgrade_cost,
        "evolve_cost":evolve_cost
    });
    file.write_all(serde_json::to_string_pretty(&upgrade_data_json).unwrap().as_bytes()).unwrap();
}

fn handle_material_json(out_dir: &str) {
    let material_json_path = "./ArknightsGameData/zh_CN/gamedata/excel/item_table.json";
    println!("cargo:rerun-if-changed={}", material_json_path);
    let mut file = File::open(material_json_path).unwrap();
    let output = File::create(format!("{}/material.json", out_dir)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let material_json: Value = serde_json::from_str(&content).unwrap();
    let material_json = material_json.as_object().unwrap();
    let material_json = material_json.get("items").unwrap().as_object().unwrap();
    let mut material_json = material_json.iter().filter(|(id, _)| {
        ((id.len() == 5 && (id.starts_with("30")||id.starts_with("31"))) || (id.len() == 4 && id.starts_with("32"))) || id.as_str() == "4001"
    }).map(|(id, item)| {
        let name = item.get("name").unwrap().as_str().unwrap();
        json!(
            {
                "id":id,
                "name":name
            }
        )
    }).collect::<Vec<Value>>();
    material_json.push(
        json!(
            {
                "id":"exp",
                "name":"经验"
            }
        )
    );
    let material_json = json!(
        {
            "materials":material_json
        }
    );
    //write material_json to output
    let mut output = std::io::BufWriter::new(output);
    output.write_all(serde_json::to_string_pretty(&material_json).unwrap().as_bytes()).unwrap();
}

fn handle_character_json(out_dir: &str) {
    let char_table_path = "./ArknightsGameData/zh_CN/gamedata/excel/character_table.json";
    println!("cargo:rerun-if-changed={}", char_table_path);
    let mut file = File::open(char_table_path).unwrap();
    let output = File::create(format!("{}/character.json", out_dir)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let char_table: Value = serde_json::from_str(&content).unwrap();
    let char_table = char_table.as_object().unwrap();
    let char_table: Vec<Value> = char_table.iter().filter_map(|(_, char)| {
        let name = char.get("name").unwrap().as_str().unwrap();
        let sub_profession = char.get("subProfessionId").unwrap().as_str().unwrap();
        let is_item = sub_profession == "notchar1" || sub_profession == "notchar2";
        if (name.contains("预备干员")) || is_item { return None; }

        let rarity = char.get("rarity").unwrap().as_u64().unwrap();
        let phases = char.get("phases").unwrap().as_array().unwrap();
        let profession = char.get("profession").unwrap().as_str().unwrap();
        if profession=="TOKEN"||profession=="TRAP" { return None; }
        let mut evolve_cost_1: Option<Vec<Value>> = None;
        let mut evolve_cost_2: Option<Vec<Value>> = None;
        let phase_count = phases.len();
        match phase_count {
            2 => {
                evolve_cost_1 = parse_evolve_cost(&phases[1].get("evolveCost").unwrap());
            }
            3 => {
                evolve_cost_1 = parse_evolve_cost(&phases[1].get("evolveCost").unwrap());
                evolve_cost_2 = parse_evolve_cost(&phases[2].get("evolveCost").unwrap());
            }
            _ => {
                if phase_count != 1 {
                    panic!("Unexpected operator phase length: {}", phase_count);
                }
            }
        }
        let skills = if rarity>2 {
            println!("{}", name);
            let skills = char.get("skills").unwrap().as_array().unwrap();
            let default = vec![];
            Some(
                skills.iter().map(|skill|{
                    let conds = skill.get("levelUpCostCond").unwrap().as_array().unwrap();
                    conds.iter().map(|cond|{
                        let cost = cond.get("levelUpCost").unwrap().as_array().unwrap_or(&default);
                        cost.iter().map(|cost|{
                            let id = cost.get("id").unwrap().as_str().unwrap();
                            let count = cost.get("count").unwrap().as_u64().unwrap();
                            json!(
                                {
                                    "id":id,
                                    "count":count
                                }
                            )
                        }).collect::<Vec<Value>>()
                    }).collect::<Vec<Vec<Value>>>()
                }).collect::<Vec<_>>()
            )
        }else {
            None
        };
        Some(
            json!(
                {
                    "name":name,
                    "rarity":rarity,
                    "profession":profession,
                    "skills":skills,
                    "evolve_cost_1":evolve_cost_1,
                    "evolve_cost_2":evolve_cost_2
                }
            )
        )
    }).collect::<Vec<Value>>();
    let char_table = json!(
        {
            "characters":char_table
        }
    );
    //write char_table to output
    let mut output = std::io::BufWriter::new(output);
    output.write_all(serde_json::to_string_pretty(&char_table).unwrap().as_bytes()).unwrap();
}

fn parse_evolve_cost(cost: &Value) -> Option<Vec<Value>> {
    let cost = cost.as_array();
    if let None = cost {
        return None;
    }
    let cost = cost.unwrap();

    let cost = cost.iter().map(|item| {
        let item = item.as_object().unwrap();
        let id = item.get("id").unwrap().as_str().unwrap().to_string();
        let count = item.get("count").unwrap().as_u64().unwrap() as u8;
        json!(
            {
                "id":id,
                "count":count
            }
        )
    }).collect::<Vec<Value>>();
    Some(cost)
}