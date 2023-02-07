use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::string::{String, ToString};
use anyhow::Error;
use futures::executor::block_on;
use serde_json::Value;
use crate::material::Material;
use crate::operator::Operator;
use crate::upgrade_cost::UpgradeCostStore;

const CHAR_TABLE_PATH:&str="character.json";

const MATERIAL_TABLE_PATH:&str="material.json";

const UPGRADE_COST_PATH:&str="upgrade_data.json";

pub struct ResourceLoader{

    json_path: String,

    pub(crate) is_operator_loaded: bool,
    operator_list: HashMap<String,Operator>,

    pub(crate) is_material_loaded: bool,
    material_list:HashMap<String,Material>,

    pub(crate) is_upgrade_data_loaded: bool,
    upgrade_cost_store:UpgradeCostStore
}

impl ResourceLoader{
    /// Create a new ResourceLoader
    pub fn new(json_path:&str) -> Self{
        let mut loader = Self{
            json_path: json_path.to_string(),
            is_operator_loaded: false,
            operator_list: HashMap::<String,Operator>::new(),
            is_material_loaded: false,
            material_list: HashMap::<String,Material>::new(),
            is_upgrade_data_loaded: false,
            upgrade_cost_store:UpgradeCostStore::new()
        };
        block_on(loader.load_resources()).expect(format!("Failed to load resources from {}",json_path).as_str());
        loader
    }

    /// Get the operator list
    /// # Errors
    /// If the operator list is not loaded, return an error
    /// # Examples
    /// ```
    /// use ark_calculator::resource_loader::ResourceLoader;
    /// let mut resource_loader = ResourceLoader::new();
    /// let operator_list = resource_loader.get_operator_list();
    /// assert!(operator_list.is_ok());
    /// ```
    /// The result is wrapped in a arc mutex, so you can clone it and use it
    pub fn get_operator_list(&self) -> anyhow::Result<&HashMap<String,Operator>>{
        if self.is_operator_loaded{
            Ok(&self.operator_list)
        }else{
            Err(Error::msg("Operator list is not loaded"))
        }
    }

    /// Get the material name by id
    /// # Errors
    /// If the material list is not loaded, return an error
    /// If the material is not found, return an error
    /// # Examples
    /// ```
    /// use ark_calculator::resource_loader::ResourceLoader;
    /// let mut resource_loader = ResourceLoader::new();
    /// let material_name = resource_loader.get_material_name("exp");
    /// assert!(material_name.is_ok());
    /// ```
    pub fn get_material_name(&self,id:&str)->anyhow::Result<String>{
        if self.is_material_loaded {
            self.material_list.get(id).ok_or(Error::msg("The specified material not found")).map(|material|material.name.clone())
        }else {
            Err(Error::msg("Material list is not loaded"))
        }
    }

    pub fn get_upgrade_store(&self)->anyhow::Result<&UpgradeCostStore>{
        if self.is_upgrade_data_loaded {
            Ok(&self.upgrade_cost_store)
        }else{
            Err(Error::msg("Upgrade cost data is not loaded"))
        }
    }

    async fn load_resources(&mut self) -> anyhow::Result<()>{

        let operator_loading = Self::load_operator(&self.json_path,&mut self.operator_list);
        let material_loading = Self::load_material(&self.json_path,&mut self.material_list);
        let upgrade_cost_loading = Self::load_upgrade_cost(&self.json_path,&mut self.upgrade_cost_store);

        if let Err(e) = operator_loading.await{
            return Err(Error::msg(format!("Failed to load operator list: {}",e)));
        };
        self.is_operator_loaded = true;
        if let Err(e) = material_loading.await{
            return Err(Error::msg(format!("Failed to load material list: {}",e)));
        };
        self.is_material_loaded = true;
        if let Err(e) = upgrade_cost_loading.await{
            return Err(Error::msg(format!("Failed to load upgrade cost data: {}",e)));
        };
        self.is_upgrade_data_loaded = true;

        Ok(())
    }

    async fn load_operator(json_path:&str,operator_list: &mut HashMap<String, Operator>)->anyhow::Result<()>{
        let path = Path::new(json_path).join(CHAR_TABLE_PATH);
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let char_table:Value = serde_json::from_str(&content)?;
        let char_table = char_table.as_object().unwrap().get("characters").unwrap().as_array().unwrap();
        for char in char_table{
            let operator = Operator::new(char);
            operator_list.insert(operator.name.clone(),operator);
        }
        Ok(())
    }

    async fn load_material(json_path:&str,material_list:&mut HashMap<String,Material>)->anyhow::Result<()>{
        let path = Path::new(json_path).join(MATERIAL_TABLE_PATH);
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let material_table:Value = serde_json::from_str(&content)?;
        let material_table = material_table.as_object().unwrap().get("materials").unwrap().as_array().unwrap();
        for material in material_table{
            let material_item = Material::new(material.get("name").unwrap().as_str().unwrap().to_string());
            material_list.insert(material.get("id").unwrap().as_str().unwrap().to_string(),material_item);
        }
        Ok(())
    }

    async fn load_upgrade_cost(json_path:&str,upgrade_cost_store:&mut UpgradeCostStore)->anyhow::Result<()>{
        let path = Path::new(json_path).join(UPGRADE_COST_PATH);
        let mut file = File::open(path)?;
        let mut content=String::new();
        file.read_to_string(&mut content)?;
        let upgrade_data_json:Value=serde_json::from_str(&content).unwrap();
        upgrade_cost_store.load(&upgrade_data_json);
        Ok(())
    }
}