<script setup lang="ts">
import MarkDownRender from "../components/MarkDownRender.vue";
import { ref } from "vue";
const formData = ref(`
# Title
# SubTitle
- list1
- list2

\`\`\` js
console.log('ttt')
\`\`\`

|A|B|
|-|-|
|1|2|

> remark

`);
</script>
<template>
  <div class="main-container">
    <div class="content">
      <a-textarea
        v-model:value="formData"
        class="full-textarea"
        :autosize="false"
      />
    </div>
    <div class="content">
      <mark-down-render :text="formData" class="markdown" />
    </div>
  </div>
</template>
<style lang="less">
@import url("/src/style/common.less");
.main-container {
  height: calc(100% - 6px);
  display: flex;
  margin: 3px;

  .content {
    width: calc(50% - 6px);
    height: calc(100% - 6px);
    flex: 1;
    overflow: auto;
    margin: 3px;
    padding: 3px;
  }
}
</style>
