<script setup lang="ts">
import { DataTableColumn, DataTableColumns, NButton, NDataTable, NDivider, NInputNumber, NList, NListItem, NScrollbar, NSelect, NSkeleton, NTooltip, NIcon, DataTableRowKey } from 'naive-ui';
import { computed, h, onMounted, reactive, ref, watch } from 'vue';
import AddTask from '../assets/AddTask.vue';
import { getMaxElite, getMaxLevel, getRarityColor } from '../helpers/OperatorHelper';
import { ProfileManager } from '../helpers/ProfileManager';
import { ResourceLoader } from '../helpers/ResourceLoader';
import { I18n } from '../i18n/strings';
import { useFeatStore, useProfileStore } from '../store';
import { OperatorTarget, SkillTarget, Stage, Material, OperatorInfo } from '../types';

const i18n = I18n.getInstance();

// profile related code

const profileStore = useProfileStore();

console.log(`loading profile ${profileStore.profile}`)

const data: OperatorTarget[] = reactive([]);

ProfileManager.loadProfile(profileStore.profile).then((newData) => {
    data.splice(0, data.length, ...newData);
})

profileStore.$subscribe((mutation, state) => {
    console.log(mutation, state)
    console.log(`loading profile ${state.profile}`)
    ProfileManager.loadProfile(state.profile).then((newData) => {
        data.splice(0, data.length, ...newData);
    })
})

// end of profile related code

const featStore = useFeatStore();

// window size related code

const windowHeight = ref(document.documentElement.clientHeight);

const bottomBarHeight = 60;

const separatorHeight = (document.getElementById('separator')?.clientHeight ?? 0) + 48;

const headerHeight = ref(document.getElementById('header')?.clientHeight ?? 0);

const profileTableHeight = ref(document.getElementById('profile-table')?.clientHeight ?? 0);

onMounted(() => {
    window.addEventListener('resize', () => {
        console.log('resize', document.documentElement.clientHeight)
        console.log('resize', document.getElementById('header')?.clientHeight ?? 0)
        console.log('resize', document.getElementById('profile-table')?.clientHeight ?? 0)
        console.log('resize', document.getElementById('bottom-bar')?.clientHeight ?? 0)
        windowHeight.value = document.documentElement.clientHeight;
        headerHeight.value = document.getElementById('header')?.clientHeight ?? 0;
        profileTableHeight.value = document.getElementById('profile-table')?.clientHeight ?? 0;
    })
})

// end of window size related code

// new operator related code

const newOperatorName = ref<string | undefined>(undefined);

const operatorList: OperatorInfo[] = reactive([]);

ResourceLoader.getOperatorList().then((newData) => {
    operatorList.splice(0, operatorList.length, ...newData);
})

const newOperatorOptions = computed(() => {
    return operatorList.filter((operator) => operator.name.includes(newOperatorName.value ?? '')).map((operator) => ({ label: operator.name, value: operator.name, style: { color: getRarityColor(operator.rarity) } }))
})

// end of new operator related code

const checkedOperators: OperatorTarget[] = reactive([]);
const hasCheckedOperators = computed(() => checkedOperators.length > 0);

const plannedStages: Stage[] = reactive([]);

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
            type: 'selection',

        },
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
        }
    ]
)

watch(
    () => featStore.useLvFeature,
    () => {
        calculateProfile();
        console.log("feat change")
        if (featStore.useLvFeature) {
            columns.splice(3, 0, lvColumns)
        } else {
            const index = columns.indexOf(lvColumns)
            if (index >= 0) {
                columns.splice(index, 1)
            }
        }
    },
    { immediate: true }
)

watch(
    () => featStore.useSkillFeature,
    () => {
        calculateProfile();
        console.log("feat change")
        if (featStore.useSkillFeature) {
            const index = featStore.useLvFeature ? 4 : 3
            columns.splice(index, 0, skillColumns)
        } else {
            const index = columns.indexOf(skillColumns)
            if (index >= 0) {
                columns.splice(index, 1)
            }
        }
    },
    { immediate: true }
)

const result = reactive<{ material: Material, count: number }[]>([])

const isPlanLoading = ref(false);

const showAddIcon = ref(false);

function calculateProfile() {
    console.log(profileStore.profile)
    ResourceLoader.calculateProfileCost(data, featStore.useLvFeature, featStore.useSkillFeature).then((cost) => {
        result.splice(0, result.length, ...Array.from(cost.entries()).map(([material, count]) => ({ material, count })));
    })
}

function checkedOperatorsChange(checkedOperatorKeys: DataTableRowKey[]) {
    checkedOperators.splice(0, checkedOperators.length);
    checkedOperatorKeys.forEach((key) => {
        const operator = data.find((operator) => operator.name === key)!;
        checkedOperators.push(operator);
    })
    if (checkedOperators.length > 0) {
        calculateSelectedOperators();
    } else {
        calculateProfile();
    }
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

function deleteSelectedOperators() {
    updateData(() => {
        checkedOperators.forEach((operator) => {
            const index = data.indexOf(operator);
            if (index >= 0) {
                data.splice(index, 1);
            }
        })
        checkedOperators.splice(0, checkedOperators.length);
    })
}

function calculateSelectedOperators() {
    ResourceLoader.calculateProfileCost(checkedOperators, featStore.useLvFeature, featStore.useSkillFeature).then((cost) => {
        result.splice(0, result.length, ...Array.from(cost.entries()).map(([material, count]) => ({ material, count })));
    })
}

function updateData(update: () => void) {
    update();
    ProfileManager.saveProfile(profileStore.profile, data);
    profileTableHeight.value = document.getElementById("profile-table")!.clientHeight;
    calculateProfile();
}

function getPlan() {
    isPlanLoading.value = true;
    plannedStages.splice(0, plannedStages.length);

    console.log("get plan")
    console.log(result)
    const required = new Map<string, number>();
    result.forEach((item) => {
        required.set(item.material.id, item.count);
    })
    ResourceLoader.getPlannerPlan(required).then((plan) => {
        isPlanLoading.value = false;
        console.log(plan);
        plan.forEach((count, stage) => {
            plannedStages.push({
                name: stage,
                count: count
            });
        })
    }).catch((e) => {
        isPlanLoading.value = false;
        console.log(e)
    })
}
</script>

<template>

    <NScrollbar class="main">
        <div>
            <NDataTable id="profile-table" class="table noselect" :columns="columns" :data="data"
                @update:checked-row-keys="checkedOperatorsChange" :row-key="(row) => row.name" :max-height="300">
                <template #empty>
                    <span class="empty">
                        {{ i18n.getStringDef("table_empty") }}
                    </span>
                </template>
            </NDataTable>
            <NDivider id="separator" title-placement="left" class="noselect">
                {{ i18n.getStringDef("result_title") }}
            </NDivider>
            <div class="row">
                <div class="sub-result">
                    <NList hoverable>
                        <NListItem class="result-item" v-for="item in result" :key="item.material.id">
                            <span>{{ item.material.name }}</span>
                            <span class="count">{{ item.count }}</span>
                        </NListItem>
                    </NList>
                </div>
                <div class="sub-result">
                    <NSkeleton text class="loading" v-if="isPlanLoading" :repeat="6"
                        :style="`max-height: ${0.5 * (windowHeight - separatorHeight - bottomBarHeight - headerHeight)}px;`" />
                    <NList v-else hoverable>
                        <NListItem class="result-item" v-for="item in plannedStages" :key="item.name">
                            <span>{{ item.name }}</span>
                            <span class="count">{{ item.count }}</span>
                        </NListItem>
                    </NList>
                </div>
            </div>
        </div>

    </NScrollbar>

    <div id="bottom-bar" class="row bottom-bar">
        <NSelect class="new-oper-select" v-model:value="newOperatorName" filterable
            :placeholder="i18n.getStringDef('select_hint')" clearable :options="newOperatorOptions"
            @update:value="handleNewOperatorValueChange" />
        <NTooltip trigger="hover" text v-if="showAddIcon" placement="top">
            <template #trigger>
                <NButton class="add-to-profile" @click="addTargetToProfile">
                    <NIcon>
                        <AddTask />
                    </NIcon>
                </NButton>
            </template>
            <span>{{ i18n.getStringDef("select_add") }}</span>
        </NTooltip>
        <div class="bottom-actions row">
            <div v-if="hasCheckedOperators">
                <NButton class="bottom-action" type="error" @click="deleteSelectedOperators">{{
                    i18n.getStringDef("btn_delete")
                }}</NButton>
            </div>
            <NButton class="bottom-action" @click="getPlan">{{
                i18n.getStringDef("btn_plan")
            }}</NButton>
        </div>

    </div>
</template>

<style scoped>
.sub-result {
    margin: 1%;
    width: 50vw;
    height: 100%;
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

.bottom-actions {
    margin-left: auto;
    margin-right: 10px;
}

.bottom-action {
    margin-left: 10px;
}

.bottom-bar {
    position: fixed;
    background-color: var(--bar-background-color);
    width: 100%;
    height: 55px;
    bottom: 0;
    margin-bottom: 0px;
}
</style>