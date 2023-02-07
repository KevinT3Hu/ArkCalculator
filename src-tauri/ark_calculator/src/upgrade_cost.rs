use serde_json::Value;

pub struct UpgradeCostStore{
    upgrade_exp_cost:Vec<Value>,
    upgrade_gold_cost:Vec<Value>,
    evolve_gold_cost:Vec<Value>
}

impl UpgradeCostStore {

    pub(crate) fn new()->Self{
        Self{
            upgrade_exp_cost:Vec::new(),
            upgrade_gold_cost:Vec::new(),
            evolve_gold_cost:Vec::new()
        }
    }

    pub(crate) fn load(&mut self,upgrade_cost_json:&Value){
        let upgrade_cost_json = upgrade_cost_json.as_object().unwrap();
        let upgrade_exp_cost=upgrade_cost_json.get("exp_cost").unwrap().as_array().unwrap().clone();
        let upgrade_gold_cost=upgrade_cost_json.get("upgrade_cost").unwrap().as_array().unwrap().clone();
        let evolve_gold_cost=upgrade_cost_json.get("evolve_cost").unwrap().as_array().unwrap().clone();
        self.upgrade_exp_cost=upgrade_exp_cost;
        self.upgrade_gold_cost=upgrade_gold_cost;
        self.evolve_gold_cost=evolve_gold_cost;
    }

    pub(crate) fn get_exp_cost(&self,elite:u8,current_level:u8,target_level:u8)->u32{
        if current_level==target_level { return 0; }
        let cost_list = &self.upgrade_exp_cost[usize::from(elite)].as_array().unwrap();
        self.get_exp_cost(elite,current_level+1,target_level)+cost_list[usize::from(current_level-1)].as_u64().unwrap() as u32
    }

    pub(crate) fn get_upgrade_cost(&self,elite:u8,current_level:u8,target_level:u8)->u32{
        if current_level==target_level { return 0; }
        let cost_list=&self.upgrade_gold_cost[usize::from(elite)].as_array().unwrap();
        self.get_upgrade_cost(elite,current_level+1,target_level)+cost_list[usize::from(current_level-1)].as_u64().unwrap() as u32
    }

    pub(crate) fn get_evolve_gold_cost(&self,rarity:u8,current_elite:u8,target_elite:u8)->u32{
        if current_elite==target_elite { return 0; }
        self.evolve_gold_cost[usize::from(rarity-2)].as_array().unwrap()[usize::from(current_elite)].as_u64().unwrap() as u32+self.get_evolve_gold_cost(rarity,current_elite+1,target_elite)
    }
}