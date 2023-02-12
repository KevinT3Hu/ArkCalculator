use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::AddAssign;
use std::str::FromStr;
use anyhow::Error;

use serde_json::Value;
use crate::skill::{parse_skills, Skills};
use crate::upgrade_cost::UpgradeCostStore;

pub type CostResult = anyhow::Result<HashMap<String,u32>>; //use u32 here because the cost of gold will overflow u8 and u16

pub struct Operator{
    pub name:String,
    pub rarity:u8,
    pub profession:Profession,
    pub skills:Option<Skills>,
    pub evolve_cost_1:Option<HashMap<String,u32>>, //key:material id, value:material count
    pub evolve_cost_2:Option<HashMap<String,u32>> //not all operator has 2nd evolve
}

impl Operator{
    pub(crate) fn new(char_json:&Value) -> Self{
        let name = char_json.get("name").unwrap().as_str().unwrap().to_string();
        let rarity = char_json.get("rarity").unwrap().as_u64().unwrap() as u8;
        let profession = char_json.get("profession").unwrap().as_str().unwrap().parse().unwrap();
        let evolve_cost_1 = Self::parse_evolve_cost(char_json.get("evolve_cost_1").unwrap());
        let evolve_cost_2 = Self::parse_evolve_cost(char_json.get("evolve_cost_2").unwrap());
        let skills = parse_skills(char_json.get("skills").unwrap());
        Self{
            name,
            rarity,
            profession,
            skills,
            evolve_cost_1,
            evolve_cost_2
        }
    }

    fn parse_evolve_cost(cost:&Value) -> Option<HashMap<String,u32>>{
        cost.as_array().map(|cost|{
            let mut cost_map = HashMap::<String,u32>::new();
            for cost in cost{
                let cost = cost.as_object().unwrap();
                let id = cost.get("id").unwrap().as_str().unwrap().to_string();
                let count = cost.get("count").unwrap().as_u64().unwrap() as u32;
                cost_map.insert(id,count);
            }
            cost_map
        })
    }

    //this will panic if the parameter is not valid because I'm lazy to check
    pub fn calculate_cost(&self,current_elite:u8,current_level:u8,target_elite:u8,target_level:u8,store:&UpgradeCostStore)->CostResult{
        let mut cost_map:HashMap<String,u32> = HashMap::new();
        let mut exp_cost=0;
        let mut gold_cost=0;
        match (current_elite,target_elite) {
            (0,0) | (1,1) | (2,2) =>{ 
                exp_cost=store.get_exp_cost(current_elite,current_level,target_level);
                gold_cost=store.get_upgrade_cost(current_elite,current_level,target_level);
            }
            (0,1)=>{
                cost_map.extend(self.evolve_cost_1.as_ref().unwrap().to_owned());
                
                exp_cost+=store.get_exp_cost(0,current_level,Self::get_elite_max_level(self.rarity,0));
                exp_cost+=store.get_exp_cost(1,1,target_level);
                
                gold_cost+=store.get_upgrade_cost(0,current_level,Self::get_elite_max_level(self.rarity,0));
                gold_cost+=store.get_upgrade_cost(1,1,target_level);
                gold_cost+=store.get_evolve_gold_cost(self.rarity,0,1);
            }
            (1,2)=>{
                cost_map.extend(self.evolve_cost_2.as_ref().unwrap().to_owned());
                
                exp_cost+=store.get_exp_cost(1,current_level,Self::get_elite_max_level(self.rarity,1));
                exp_cost+=store.get_exp_cost(2,1,target_level);
                
                gold_cost+=store.get_upgrade_cost(1,current_level,Self::get_elite_max_level(self.rarity,1));
                gold_cost+=store.get_upgrade_cost(2,1,target_level);
                gold_cost+=store.get_evolve_gold_cost(self.rarity,1,2);
            }
            (0,2)=>{
                cost_map.extend(self.evolve_cost_1.as_ref().unwrap().to_owned());
                cost_map.extend(self.evolve_cost_2.as_ref().unwrap().to_owned());
                
                exp_cost+=store.get_exp_cost(0,current_level,Self::get_elite_max_level(self.rarity,0));
                exp_cost+=store.get_exp_cost(1,1,Self::get_elite_max_level(self.rarity,1));
                exp_cost+=store.get_exp_cost(2,1,target_level);
                
                gold_cost+=store.get_upgrade_cost(0,current_level,Self::get_elite_max_level(self.rarity,0));
                gold_cost+=store.get_upgrade_cost(1,1,Self::get_elite_max_level(self.rarity,1));
                gold_cost+=store.get_upgrade_cost(2,1,target_level);
                gold_cost+=store.get_evolve_gold_cost(self.rarity,0,2);
            }
            
            _=>{ panic!("elite level outbound") }
        }
        cost_map.insert(String::from("exp"),exp_cost);
        cost_map.insert(String::from("4001"),gold_cost);
        Ok(cost_map)
    }

    pub fn calculate_skill_cost(&self,skill_index:u8,current_level:u8,target_level:u8)->CostResult{
        if current_level==target_level { return Ok(HashMap::new()); }
        let skills = &self.skills.clone().unwrap_or(vec![]);
        if skill_index>=skills.len() as u8{
            return Err(Error::msg("skill index outbound"));
        }
        let mut cost_map:HashMap<String,u32> = HashMap::new();
        let skill = &skills[skill_index as usize];
        let cost = &skill[current_level as usize];
        cost_map = combine(&cost_map,cost);
        Ok(combine(&cost_map,&(self.calculate_skill_cost(skill_index,current_level+1,target_level)?)))
    }

    pub fn get_elite_max_level(rarity:u8,elite:u8)->u8{
        match (rarity,elite) {
            (0,_) | (1,_)=>{ 30 }
            (2,0)=>{ 40 }
            (2,1)=>{ 55 }
            (3,0)=>{ 45 }
            (3,1)=>{ 60 }
            (3,2)=>{ 70 }
            (4,0) | (5,0)=>{ 50 }
            (4,1)=>{ 70 }
            (4,2) | (5,1)=>{ 80 }
            (5,2)=>{ 90 }
            _=>{ panic!("elite level outbound") }
        }
    }
}

#[derive(Eq,PartialEq,Copy, Clone)]
pub enum Profession{
    CASTER, //术士
    MEDIC, //医疗
    WARRIOR, //近卫
    SUPPORT, //辅助
    SNIPER, //狙击
    SPECIAL, //特种
    PIONEER, //先锋
    TANK, //重装
}

impl FromStr for Profession{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CASTER" | "术士" => Ok(Profession::CASTER),
            "MEDIC" | "医疗" => Ok(Profession::MEDIC),
            "WARRIOR" | "近卫" => Ok(Profession::WARRIOR),
            "SUPPORT" | "辅助" => Ok(Profession::SUPPORT),
            "SNIPER" | "狙击" => Ok(Profession::SNIPER),
            "SPECIAL" | "特种" => Ok(Profession::SPECIAL),
            "PIONEER" | "先锋" => Ok(Profession::PIONEER),
            "TANK" | "重装" => Ok(Profession::TANK),
            _ => Err("profession not found".to_string())
        }
    }
}

impl Display for Profession{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Profession::CASTER => write!(f, "CASTER"),
            Profession::MEDIC => write!(f, "MEDIC"),
            Profession::WARRIOR => write!(f, "WARRIOR"),
            Profession::SUPPORT => write!(f, "SUPPORT"),
            Profession::SNIPER => write!(f, "SNIPER"),
            Profession::SPECIAL => write!(f, "SPECIAL"),
            Profession::PIONEER => write!(f, "PIONEER"),
            Profession::TANK => write!(f, "TANK"),
        }
    }
}

fn combine<T,K>(a:&HashMap<T,K>,b:&HashMap<T,K>)->HashMap<T,K>
    where T:Eq+Hash+Clone,K:AddAssign+Clone+Default{
    let mut result = a.clone();
    for (k,v) in b{
        let value = result.entry(k.clone()).or_insert(K::default());
        *value+=v.clone();
    }
    result
}