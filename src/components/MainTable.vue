<script setup lang="ts">
import { DataTableColumn, DataTableColumns, NButton, NInput, NInputNumber } from 'naive-ui';
import { TableBaseColumn } from 'naive-ui/es/data-table/src/interface';
import { h, onMounted, onUpdated, reactive, ref, watch, watchEffect } from 'vue';
import AddTask from '../assets/AddTask.vue';
import { getMaxElite, getMaxLevel, getRarityColor } from '../helpers/OperatorHelper';
import { ProfileManager } from '../helpers/ProfileManager';
import { ResourceLoader } from '../helpers/ResourceLoader';
import { I18n } from '../i18n/strings';
import { OperatorTarget, SkillTarget } from '../types';

const i18n = I18n.getInstance();

const props = defineProps<{
    profile: string,
    useLvFeature: boolean,
    useSkillFeature: boolean,
}>()

console.log(`loading profile ${props.profile}`)

const profile = ref(props.profile);

const profileManager = await ProfileManager.getProfileManager();
const data = reactive(await profileManager.loadProfile(props.profile));

watch(
    () => props.profile,
    async (newProfile) => {
        console.log(`loading profile ${newProfile}`)
        const newData = await profileManager.loadProfile(newProfile);
        data.splice(0, data.length, ...newData);
    }
)

const windowHeight = ref(document.documentElement.clientHeight);

const bottomBarHeight = 100;

const separatorHeight = document.getElementById('separator')?.clientHeight ?? 0 + 48;

const headerHeight = ref(document.getElementById('header')?.clientHeight ?? 0);

onMounted(() => {
    window.addEventListener('resize', () => {
        windowHeight.value = document.documentElement.clientHeight;
        headerHeight.value = document.getElementById('header')?.clientHeight ?? 0;
    })
})

const newOperatorName = ref<string | undefined>(undefined);

const operatorList = await ResourceLoader.getOperatorList();

const newOperatorOptions = ref(operatorList.map((operator) => ({ 
    label: operator.name, 
    value: operator.name,
    style: {
        color: getRarityColor(operator.rarity)
    }
})))

watch(newOperatorName, (value) => {
    newOperatorOptions.value = operatorList.filter((operator) => operator.name.includes(value ?? '')).map((operator) => ({ label: operator.name, value: operator.name, style: { color: getRarityColor(operator.rarity) } }))
})

const skillColumnsChilden = [];

for (let i = 0; i < 3; i++) {
    skillColumnsChilden.push({
        title: i18n.getStringWithTemplate('table_head_skill_with_index', `${i + 1}`),
        key: 'skill' + i,
        children: [
            {
                title: i18n.getStringDef('table_head_skill_celite'),
                key: 'skill' + i + 'CurrentElite',
                render(row: OperatorTarget, index: number) {
                    if (row.skill_targets.length <= i) {
                        return h('span', '')
                    }
                    return h(NInputNumber, {
                        value: row.skill_targets[i].current_elite,
                        min: 0,
                        max: getMaxElite(row.rarity),
                        onUpdateValue: (value) => {
                            updateData(() => {
                                data[index].skill_targets[i].current_elite = value ?? 0;
                                if (row.skill_targets[i].target_elite < row.skill_targets[i].current_elite) {
                                    data[index].skill_targets[i].target_elite = row.skill_targets[i].current_elite;
                                }
                            })
                        }
                    })
                }
            },
            {
                title: i18n.getStringDef('table_head_skill_telite'),
                key: 'skill' + i + 'TargetElite',
                render(row: OperatorTarget, index: number) {
                    if (row.skill_targets.length <= i) {
                        return h('span', '')
                    }
                    return h(NInputNumber, {
                        value: row.skill_targets[i].target_elite,
                        min: row.skill_targets[i].current_elite,
                        max: 3,
                        onUpdateValue: (value) => {
                            updateData(() => {
                                data[index].skill_targets[i].target_elite = value ?? 0;
                            })
                        }
                    })
                }
            }
        ]
    })
}

const skillColumns: DataTableColumn<OperatorTarget> = {
    title: i18n.getStringDef("table_head_skills"),
    key: 'skills',
    children: skillColumnsChilden
}

const lvColumns: DataTableColumn<OperatorTarget> = {
    title: i18n.getStringDef("table_head_lv"),
    key: 'lv',
    children: [
        {
            title: i18n.getStringDef("table_head_celite"),
            key: 'currentElite',
            render(row, index) {
                return h(NInputNumber, {
                    value: row.current_elite,
                    min: 0,
                    max: getMaxElite(row.rarity),
                    onUpdateValue: (value) => {
                        updateData(() => {
                            data[index].current_elite = value ?? 0;
                            if ((row.target_elite == row.current_elite) && (row.target_level < row.current_level)) {
                                data[index].target_level = data[index].current_level;
                            }
                            if (row.current_level > getMaxLevel(row.rarity, row.current_elite)) {
                                data[index].current_level = getMaxLevel(row.rarity, row.current_elite);
                            }
                            if (row.target_elite < row.current_elite) {
                                data[index].target_elite = row.current_elite;
                            }
                        })
                    }
                })
            }
        },
        {
            title: i18n.getStringDef("table_head_clevel"),
            key: 'currentLevel',
            render(row, index) {
                return h(NInputNumber, {
                    value: row.current_level,
                    min: 1,
                    max: getMaxLevel(row.rarity, row.current_elite),
                    onUpdateValue: (value) => {
                        updateData(() => {
                            data[index].current_level = value ?? 1;
                            if ((row.target_elite == row.current_elite) && (row.target_level < row.current_level)) {
                                data[index].target_level = data[index].current_level;
                            }
                        })
                    }
                })
            }
        },
        {
            title: i18n.getStringDef("table_head_telite"),
            key: 'targetElite',
            render(row, index) {
                return h(NInputNumber, {
                    value: row.target_elite,
                    min: row.current_elite,
                    max: getMaxElite(row.rarity),
                    onUpdateValue: (value) => {
                        updateData(() => {
                            data[index].target_elite = value ?? 0;
                            if (row.current_elite == row.target_elite) {
                                if (row.target_level < row.current_level) {
                                    data[index].target_level = data[index].current_level;
                                }
                            } else {
                                data[index].target_level = 1;
                            }
                            if (row.target_level > getMaxLevel(row.rarity, row.target_elite)) {
                                data[index].target_level = getMaxLevel(row.rarity, row.target_elite);
                            }
                        })
                    }
                })
            }
        },
        {
            title: i18n.getStringDef("table_head_tlevel"),
            key: 'targetLevel',
            render(row, index) {
                var min: number;
                if (row.target_elite == row.current_elite) {
                    min = row.current_level;
                } else {
                    min = 1;
                }
                return h(NInputNumber, {
                    value: row.target_level,
                    min: min,
                    max: getMaxLevel(row.rarity, row.target_elite),
                    onUpdateValue: (value) => {
                        updateData(() => {
                            data[index].target_level = value ?? 1;
                        })
                    }
                })
            }
        }
    ]
}

const columns: DataTableColumns<OperatorTarget> = reactive(
    [
        {
            title: i18n.getStringDef("table_head_name"),
            key: 'name',
            render(row) {
                //render with color
                return h('span', {
                    style: {
                        color: getRarityColor(row.rarity)
                    }
                }, row.name)
            }
        },
        {
            title: i18n.getStringDef("table_head_rarity"),
            key: 'rarity',
            render(row) {
                return h('span', row.rarity + 1)
            }
        },
        {
            title: i18n.getStringDef("table_head_action"),
            key: 'action',
            render(_row, index) {
                return h(NButton, {
                    onClick: () => {
                        updateData(() => {
                            data.splice(index, 1);
                        })
                    }
                }, i18n.getStringDef("table_action_delete"))
            }
        }
    ]
)

watch(
    () => props.useLvFeature,
    () => {
        calculateProfile();
        console.log("feat change")
        if (props.useLvFeature) {
            columns.splice(2, 0, lvColumns)
        } else {
            const index = columns.indexOf(lvColumns)
            if(index>=0){
                columns.splice(index, 1)
            }
        }
    },
    { immediate: true }
)

watch(
    () => props.useSkillFeature,
    () => {
        calculateProfile();
        console.log("feat change")
        if (props.useSkillFeature) {
            const index = props.useLvFeature ? 3 : 2
            columns.splice(index, 0, skillColumns)
        } else {
            const index = columns.indexOf(skillColumns)
            if(index>=0){
                columns.splice(index, 1)
            }
        }
    },
    { immediate: true }
)

const result = ref<{ name: string, count: number }[]>([])

const showAddIcon = ref(false);

function handleSearch(query: string) {
    newOperatorOptions.value = operatorList.filter((operator) => operator.name.includes(query)).map((operator) => ({ label: operator.name, value: operator.name, style: { color: getRarityColor(operator.rarity) } }))
    newOperatorOptions.value = newOperatorOptions.value.filter((operator) => !data.find((target) => target.name === operator.value))
}

function calculateProfile() {
    console.log(profile.value)
    ResourceLoader.calculateProfileCost(data,props.useLvFeature,props.useSkillFeature).then((cost) => {
        result.value = [];
        console.log(cost)
        console.log(Object.entries(cost))
        for (const [name, count] of cost.entries()) {
            result.value.push({ name, count })
        }
    })
}

function handleNewOperatorValueChange(value: string) {
    showAddIcon.value = operatorList.filter((operator) => data.find((target) => target.name === operator.name) === undefined).find((operator) => operator.name === value) !== undefined;
    console.log(showAddIcon.value)
}

function addTargetToProfile() {
    updateData(() => {
        if (newOperatorName.value === undefined) return;
        const operator = operatorList.find((operator) => operator.name === newOperatorName.value)!;
        const skillTargets: SkillTarget[] = [];
        for (let i = 0; i < operator.skill_count; i++) {
            skillTargets.push({
                current_elite: 0,
                target_elite: 0
            })
        }
        data.push({
            name: newOperatorName.value,
            rarity: operator.rarity,
            current_elite: 0,
            current_level: 1,
            target_elite: 0,
            target_level: 1,
            skill_targets: skillTargets
        })
    })
    newOperatorName.value = undefined;
    showAddIcon.value = false;
}

function updateData(update: () => void) {
    update();
    profileManager.saveProfile(profile.value, data);
    calculateProfile();
}

</script>

<template>

    <n-data-table class="table noselect" :columns="columns" :data="data"
        :max-height="0.4 * (windowHeight - separatorHeight - bottomBarHeight - headerHeight)">
        <template #empty>
            <span class="empty">
                {{ i18n.getStringDef("table_empty") }}
            </span>
        </template>
    </n-data-table>
    <n-divider id="separator" title-placement="left" class="noselect">
        {{ i18n.getStringDef("result_title") }}
    </n-divider>
    <div class="column result">
        <n-scrollbar
            :style="`max-height: ${0.5 * (windowHeight - separatorHeight - bottomBarHeight - headerHeight)}px;`">
            <n-list hoverable>
                <n-list-item class="result-item" v-for="item in result" :key="item.name">
                    <span>{{ item.name }}</span>
                    <span class="count">{{ item.count }}</span>
                </n-list-item>
            </n-list>
        </n-scrollbar>
    </div>
    <div class="row bottom-bar">
        <n-select class="new-oper-select" v-model:value="newOperatorName" filterable
            :placeholder="i18n.getStringDef('select_hint')" clearable :options="newOperatorOptions"
            @search="handleSearch" @update:value="handleNewOperatorValueChange" />
        <n-tooltip trigger="hover" text v-if="showAddIcon" placement="top">
            <template #trigger>
                <n-button class="add-to-profile" @click="addTargetToProfile">
                    <AddTask />
                </n-button>
            </template>
            <span>{{ i18n.getStringDef("select_add") }}</span>
        </n-tooltip>
        <n-button class="btn-calculate" type="primary" @click="calculateProfile">{{
            i18n.getStringDef("btn_calculate")
        }}</n-button>
    </div>
</template>

<style scoped>
.result-item {
    width: 50%;
}

.empty {
    font-size: 20px;
    color: gray;
}

.count {
    float: right;
}

.new-oper-select {
    margin-left: 10px;
    width: 300px;
}

.add-to-profile {
    font-size: 24px;
    margin-left: 10px;
}

.btn-calculate {
    margin-left: auto;
    margin-right: 10px;
}

.bottom-bar {
    position: absolute;
    background-color: var(--bar-background-color);
    width: 100%;
    height: 60px;
    bottom: 0;
}
</style>