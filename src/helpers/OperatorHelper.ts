import { invoke } from "@tauri-apps/api";

export function getMaxElite(rarity:number):number{
    if(rarity>=3){
        return 2;
    }

    if(rarity>=2){
        return 1;
    }

    return 0;
}

export function getMaxLevel(rarity:number,elite:number):number{
    if(rarity==0||rarity==1){
        return 30;
    }

    if(rarity==2){
        if(elite==0){
            return 40;
        }
        else{
            return 55;
        }
    }

    if(rarity==3){
        switch(elite){
            case 0:return 45;
            case 1:return 60;
            case 2:return 70;
        }
    }

    if(rarity==4){
        switch(elite){
            case 0:return 50;
            case 1:return 70;
            case 2:return 80;
        }
    }

    if(rarity==5){
        switch(elite){
            case 0:return 50;
            case 1:return 80;
            case 2:return 90;
        }
    }

    throw new Error("Invalid rarity or elite"+rarity+" "+elite);
}

export function getRarityColor(rarity:number){
    switch(rarity){
        case 3:return "#A231FF";
        case 4:return "#CC7A00";
        case 5:return "#EE5700";
        default:return "#000000";
    }
}