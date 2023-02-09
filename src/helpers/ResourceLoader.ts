import { invoke } from "@tauri-apps/api";
import { Material, OperatorInfo, OperatorTarget, Stage } from "../types";

export class ResourceLoader{

    public static async getOperatorList():Promise<OperatorInfo[]> {
        return invoke("get_operator_list")
    }

    public static async calculateProfileCost(operatorList:OperatorTarget[],useLv:boolean=true,useSkill:boolean=true):Promise<Map<Material,number>>{
        const costMap:Map<string,number> = await invoke("calculate_total_cost",{
            targets:operatorList,
            useLv:useLv,
            useSkill:useSkill
        });
        console.log(costMap);
        const costMapResult:Map<Material,number> = new Map();
        for(const [id,count] of Object.entries(costMap)){
            const name:string = await invoke("get_material_name",{materialId:id});
            costMapResult.set({id,name},count);
        }
        return Promise.resolve(costMapResult);
    }

    public static async getPlannerPlan(required:Map<string,number>):Promise<Map<string,number>>{
        const requiredWithName:Map<string,number> = new Map();
        for (const [key, value] of required.entries()) {
            if(value==0||ResourceLoader.shouldKeyBeDeleted(key)){
                required.delete(key);
            }else{
                const name:string = await invoke("get_material_name",{materialId:key});
                requiredWithName.set(name,value);
            }
        }
        const requiredString = JSON.stringify(Array.from(requiredWithName.entries()));
        console.log(requiredString);
        const plan:Promise<Stage[]> = invoke("get_planner_plan",{required:requiredString});
        return plan.then((stageList:Stage[])=>{
            console.log(stageList);
            const planMap:Map<string,number> = new Map();
            stageList.forEach((stage:Stage)=>{
                planMap.set(stage.name,stage.count);
            });
            return planMap;
        });
    }

    static shouldKeyBeDeleted(key:string):boolean{
        return key=="exp"||key.length==4;
    }
}