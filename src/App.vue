<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { Menus } from "./menus";
import router from "./router";
import { theme } from "ant-design-vue";
import { MenuProps } from "ant-design-vue";

let isCollapseMenu = ref(false);
let width = computed(() => (isCollapseMenu.value ? "50px" : "200px"));
/** 默认页面 */
const defaultPage = "/nothing";
/** 当前选中的菜单 */
const currentMenu = ref<string[]>([defaultPage]);
/** 菜单搜索条件 */
const search = ref("");
/** 菜单 */
const menu = ref<MenuProps["items"]>(Menus);
/** 使用的主题算法 */
const themeAlgorithm = ref(theme.defaultAlgorithm);

type IMenuClick = { key: string };

function onMenuClick(menu: IMenuClick) {
    const routes = router.getRoutes();
    const path =
        routes.find((item) => item.name === menu.key)?.path ?? defaultPage;
    router.push(path);
}

/**筛选目录 */
function onSearchMenu() {
    if (!Menus) return;
    menu.value = Menus.filter((item: any) => {
        return (
            item &&
            item.children.some((child: any) =>
                child.label.includes(search.value),
            )
        );
    }).map((item: any) => {
        let result = { ...item };
        result.children = item.children.filter((child: any) =>
            child.label.includes(search.value),
        );
        return result;
    });
}

/** 更换主题 */
function changeTheme(isDark: boolean) {
    if (isDark) {
        themeAlgorithm.value = theme.darkAlgorithm;
    } else {
        themeAlgorithm.value = theme.defaultAlgorithm;
    }
}
/** 获取系统配色方案 */
const colorSchema = window.matchMedia("(prefers-color-scheme: dark)");
/** 监听系统配色方案是否改变，以切换浅色/暗黑主题 */
colorSchema.addEventListener("change", (event) => {
    changeTheme(event.matches);
});

onMounted(() => {
    changeTheme(colorSchema.matches);
    router.push(defaultPage);
});
</script>

<template>
    <a-config-provider
        component-size="middle"
        :from="{ colon: true }"
        :theme="{ algorithm: themeAlgorithm }"
    >
        <a-layout style="height: 100%">
            <a-layout-sider :width="width" theme="light" class="sidebar">
                <a-input
                    placeholder="搜索目录"
                    v-model:value="search"
                    @change="onSearchMenu"
                />
                <a-menu
                    :items="menu"
                    mode="vertical"
                    v-model:selectedKeys="currentMenu"
                    @click="onMenuClick"
                    class="ant-menu"
                />
            </a-layout-sider>
            <a-layout-content>
                <router-view />
            </a-layout-content>
        </a-layout>
    </a-config-provider>
</template>

<style scoped></style>
<style lang="less">
html,
body,
#app {
    height: 100%;
    margin: 0;
}

.sidebar {
    padding-left: 1rem;
    height: 100%;
}

.ant-menu {
    height: calc(100% - 50px);
    overflow: auto;
}

.menu-group {
    font-size: 1rem;
    font-weight: 700;
    margin-bottom: 8px;
    line-height: 24px;
}

// 目录中间的分割线基础样式
.menu-base {
    line-height: 1.5;
    font-size: 0.9rem;
    border-radius: 8px;
    margin: 0;
    color: black;
    text-decoration: none;
}

.menu {
    .menu-base;
    margin: 10px, 16px;
    line-height: 3;
    height: 100%;
}
.sub-menu {
    padding: 10px 16px;
    display: block;
    .menu-base;
}
.menu-select {
    color: #409eff;
    background-color: rgba(64, 158, 255, 0.1);
    text-decoration: underline;
}
:is(.menu, .sub-menu):hover {
    .menu-select;
}

:is(.menu, .sub-menu):active {
    .menu-select;
}

.divider-base {
    margin: 0;
    padding: 0;
}

.divider-vertical {
    .divider-base;
    height: auto;
}

.divider-horizontal {
    .divider-base;
    height: 1px;
}
</style>
