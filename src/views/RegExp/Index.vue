<script setup lang="ts"></script>
<template>
  <div class="main">
    <div class="actions">
      <a-input placeholder="请输入正则表达式" />
      <a-button type="primary" ghost>复制结果</a-button>
      <a-button type="primary" ghost danger>清空</a-button>
    </div>
    <div class="match-content">
      <a-textarea />
    </div>
    <div class="match-result"></div>
  </div>
</template>
<style lang="less" scoped></style>
