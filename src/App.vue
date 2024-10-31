<script setup lang="ts">
import { ElConfigProvider } from "element-plus";
import { ref, computed } from "vue";
import { Menus } from "./menus";

let isCollapseMenu = ref(false);
let width = computed(() => (isCollapseMenu.value ? "50px" : "150px"));

const size = "default";
const zIndex = 3000;
let menus = Menus;
</script>

<template>
  <el-config-provider :size="size" :z-index="zIndex">
    <el-container style="height: 100%">
      <el-aside :width="width" style="margin-left: 1em">
        <el-scrollbar index="1">
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
                <el-divider class="divider-horizontal" />
              </div>
              <div v-else>
                <router-link :to="{ name: item.link }" class="link">
                  {{ item.title }}
                </router-link>
                <el-divider class="divider-horizontal" />
              </div>
            </div>
          </div>
        </el-scrollbar>
      </el-aside>
      <el-divider direction="vertical" class="divider-vertical" />
      <el-main>
        <router-view />
      </el-main>
    </el-container>
  </el-config-provider>
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
