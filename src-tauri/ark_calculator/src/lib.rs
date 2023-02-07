pub mod resource_loader;
pub mod material;
pub mod operator;
pub mod upgrade_cost;
pub mod skill;


#[cfg(test)]
mod test{
    use crate::resource_loader::ResourceLoader;

    const JSON_PATH:&str="../json";

    #[test]
    pub fn test_same_elite_upgrade(){
        let resource = ResourceLoader::new(JSON_PATH);
        let store = resource.get_upgrade_store().unwrap();
        for (_,oper) in resource.get_operator_list().unwrap().iter(){
            let cost = oper.calculate_cost(0,1,0,1,&store).unwrap();
            assert_eq!(cost.get("4001").unwrap().clone(), 0)
        }
    }

    #[test]
    pub fn test_level_upgrade_from_0_to_1(){
        let resource = ResourceLoader::new(JSON_PATH);
        let store = resource.get_upgrade_store().unwrap();
        for (_,oper) in resource.get_operator_list().unwrap().iter(){
            let cost = oper.calculate_cost(0,1,0,2,&store).unwrap();
            assert_eq!(cost.get("4001").unwrap().clone(), 30)
        }
    }

    #[test]
    pub fn test_upgrade_sixstar_operator_from_0_to_max(){
        let resource = ResourceLoader::new(JSON_PATH);
        let store = resource.get_upgrade_store().unwrap();
        let operator_list = resource.get_operator_list().unwrap();
        let oper=operator_list.get("能天使").unwrap();
        let cost = oper.calculate_cost(0,1,2,90,&store).unwrap();
        println!("{:?}",cost);
        assert_eq!(cost.get("4001").unwrap().clone(),1334796)
    }

    #[test]
    pub fn test_upgrade_skill_not_empty(){
        let resource = ResourceLoader::new(JSON_PATH);
        let operator_list = resource.get_operator_list().unwrap();
        let oper=operator_list.get("能天使").unwrap();
        let res = oper.calculate_skill_cost(2,0,3).unwrap();
        println!("{:?}",res);
        assert!(!res.is_empty())
    }
}
