<script setup lang="ts"></script>
<template>
  <a-button v-bind="$attrs" class="btn-common" ghost>
    <slot></slot>
  </a-button>
</template>
<style lang="less" scoped>
@import url("/src/style/base/button.less");
</style>
