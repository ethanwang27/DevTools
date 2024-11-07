<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { Menus } from "./menus";
import router from "./router";
import { theme } from "ant-design-vue";

let isCollapseMenu = ref(false);
let width = computed(() => (isCollapseMenu.value ? "50px" : "200px"));
const defaultPage = "/nothing";
const currentMenu = ref<string[]>([defaultPage]);

type IMenuClick = { key: string };

function onMenuClick(menu: IMenuClick) {
  const routes = router.getRoutes();
  const path =
    routes.find((item) => item.name === menu.key)?.path ?? defaultPage;
  router.push(path);
}

const themeAlgorithm = ref(theme.defaultAlgorithm);

function changeTheme(isDark: boolean) {
  if (isDark) {
    themeAlgorithm.value = theme.darkAlgorithm;
  } else {
    themeAlgorithm.value = theme.defaultAlgorithm;
  }
}

const colorSchema = window.matchMedia("(prefers-color-scheme: dark)");
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
        <a-menu
          :items="Menus"
          mode="vertical"
          v-model:selectedKeys="currentMenu"
          @click="onMenuClick"
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
