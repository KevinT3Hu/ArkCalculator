import { invoke } from "@tauri-apps/api";
import { OperatorInfo, OperatorTarget } from "../types";

export class ResourceLoader{

    public static async getOperatorList():Promise<OperatorInfo[]> {
        return invoke("get_operator_list")
    }

    public static async calculateProfileCost(operatorList:OperatorTarget[],useLv:boolean=true,useSkill:boolean=true):Promise<Map<string,number>>{
        const costMap:Map<string,number> = await invoke("calculate_total_cost",{
            targets:operatorList,
            useLv:useLv,
            useSkill:useSkill
        });
        console.log(costMap);
        const costMapWithName:Map<string,number> = new Map();
        for(const [id,count] of Object.entries(costMap)){
            const name:string = await invoke("get_material_name",{materialId:id});
            costMapWithName.set(name,count);
        }
        return Promise.resolve(costMapWithName);
    }
}