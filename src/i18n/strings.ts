const strings:{[key:string]:{[key:string]:string}}={
    "en":{
        "app_name":"Arknights Operator Cost Calculator",
        "head":"ArkCalculator",
        "table_head_name":"Name",
        "table_head_rarity":"Rarity",
        "table_head_celite":"Current Elite",
        "table_head_clevel":"Current Level",
        "table_head_telite":"Target Elite",
        "table_head_tlevel":"Target Level",
        "table_action_delete":"Delete",
        "tooltip_delete_profile":"Delete the selected profile",
        "tooltip_add_profile":"Add a new profile",
        "select_hint":"Select an operator to add",
        "select_add":"Click to add",
        "btn_calculate":"Calculate",
        "result_title":"Result",
    },
    "zh-CN":{
        "app_name":"明日方舟干员培养计算器",
        "table_head_name":"干员名",
        "table_head_rarity":"稀有度",
        "table_head_celite":"当前精英等级",
        "table_head_clevel":"当前等级",
        "table_head_telite":"目标精英等级",
        "table_head_tlevel":"目标等级",
        "table_action_delete":"删除",
        "tooltip_delete_profile":"删除选中的配置",
        "tooltip_add_profile":"添加新的配置",
        "select_hint":"选择添加一个干员",
        "select_add":"添加",
        "btn_calculate":"计算",
        "result_title":"结果",
    }
}

export class I18n{

    private static _instance:I18n|undefined;

    private lang:string = "en";
    private fallback:string = "en";


    private constructor(){}

    static getInstance():I18n{
        if(!this._instance){
            this._instance = new I18n();
        }
        return this._instance;
    }

    init(lang:string,fallback:string){
        this.lang = lang;
        this.fallback = fallback;
    }

    getString(key:string, lang:string):string{
        return strings[lang][key];
    }

    getStringDef(key:string):string{
        return this.getString(key, this.lang) || this.getString(key, this.fallback);
    }
}