//types to interact with the rust backend
export interface OperatorTarget{
    name:string,
    rarity:number,
    current_elite:number,
    current_level:number,
    target_elite:number,
    target_level:number,
}

export interface OperatorInfo{
    name:string,
    rarity:number,
}