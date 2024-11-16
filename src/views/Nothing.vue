<script setup lang="ts"></script>
<template>
  <div class="main">
    <a-empty description="请点击左侧菜单选择工具" class="empty" />
  </div>
</template>
<style lang="less" scoped>
.main {
  height: 100%;
  position: relative;

  .empty {
    position: absolute;
    top: 40%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
}
</style>
