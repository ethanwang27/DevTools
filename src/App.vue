<script setup lang="ts">
import { ref, computed } from "vue";
import { Menus } from "./menus";

let isCollapseMenu = ref(false);
let width = computed(() => (isCollapseMenu.value ? "50px" : "150px"));
let menus = Menus;
</script>

<template>
  <a-config-provider component-size="middle" :from="{ colon: true }">
    <a-layout style="height: 100%">
      <a-layout-sider :width="width" theme="light">
        <div :collapse="isCollapseMenu" :router="true">
          <div v-for="(item, key) in menus" :key="key">
            <div v-if="item.subMenu">
              <p class="sidebar-group-title">{{ item.title }}</p>
              <router-link
                v-for="(sub, subKey) in item.subMenu"
                :key="subKey"
                :to="{ name: sub.link }"
                class="sub-link"
              >
                {{ sub.title }}
              </router-link>
              <a-divider class="divider-horizontal" />
            </div>
            <div v-else>
              <router-link :to="{ name: item.link }" class="link">
                {{ item.title }}
              </router-link>
              <a-divider class="divider-horizontal" />
            </div>
          </div>
        </div>
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

.sidebar-group-title {
  font-size: 1rem;
  font-weight: 700;
  margin-bottom: 8px;
  line-height: 24px;
}

.line-base {
  line-height: 1.5;
  font-size: 0.9rem;
  border-radius: 8px;
  margin: 0;
  color: black;
  text-decoration: none;
}
.link {
  .line-base;
  padding: 0, 16px;
  line-height: 3;
}
.sub-link {
  padding: 10px 16px;
  display: block;
  .line-base;
}

.link-select {
  color: #409eff;
  background-color: rgba(64, 158, 255, 0.1);
  text-decoration: underline;
}
:is(.link, .sub-link):hover {
  .link-select;
}

:is(.link, .sub-link):active {
  .link-select;
}

.router-link-active {
  .link-select;
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
