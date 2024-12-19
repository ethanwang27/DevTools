<script setup lang="ts">
import { reactive } from "vue";
import { detect } from "../../utils/languageDetection";

const state = reactive({
    code: "",
    language: "",
});

async function detectLanguage() {
    if (!state.code) return;
    const result = await detect(state.code);
    state.language = result;
}

// async function formatCode() {
//     return "";
// }
</script>
<template>
    <div class="main-container">
        <div class="input-content">
            <a-textarea class="full-textarea" v-model:value="state.code" />
        </div>
        <div class="action">
            <t-button type="primary" @click="detectLanguage">自动检测</t-button>
        </div>
        <label>检测结果：{{ state.language }}</label>
    </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");
.input-content {
    width: 100%;
    height: 50%;
}
</style>
