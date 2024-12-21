<script setup lang="ts">
import { reactive } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import log from "../../utils/logger";

interface IState {
  inputStr: string;
  resultStr: string;
  inputTextareaStatus: "error" | "";
}

const state = reactive<IState>({
  inputStr: "",
  resultStr: "",
  inputTextareaStatus: "",
});

async function copyToClipboard() {
  if (!state.resultStr) return;
  await writeText(state.resultStr);
  message.info("复制成功");
}

function encode() {
  if (!state.inputStr) {
    message.warn("请输入需要编码的Url");
    state.inputTextareaStatus = "error";
    return;
  }
  invoke("url_encode", {
    data: state.inputStr,
  })
    .then((result) => {
      state.resultStr = result as string;
      message.info("编码成功");
    })
    .catch((err) => {
      message.error("编码失败");
      log(`URL编码失败: ${err}`, "error");
    });
}

function decode() {
  if (!state.inputStr) {
    message.warn("请输入需要解码的Url");
    state.inputTextareaStatus = "error";
    return;
  }
  invoke("url_decode", {
    data: state.inputStr,
  })
    .then((result) => {
      state.resultStr = result as string;
      message.info("编码成功");
    })
    .catch((err) => {
      message.error("编码失败");
      log(`URL解码失败: ${err}`, "error");
    });
}

function clear() {
  state.inputStr = "";
  state.resultStr = "";
}
</script>
<template>
  <div class="main-container vertical-two-split-container">
    <div class="split-content">
      <a-textarea
        class="full-textarea"
        v-model:value="state.inputStr"
        @change="() => (state.resultStr = '')"
        placeholder="请输入需要编码或解码的Url"
      />
    </div>
    <div class="action">
      <t-button
        @click="clear"
        type="primary"
        :disabled="!state.inputStr && !state.resultStr"
        danger
        >清空</t-button
      >
      <t-button
        @click="copyToClipboard"
        type="primary"
        :disabled="!state.resultStr"
        >复制</t-button
      >
      <t-button @click="encode" type="primary" :disabled="!state.inputStr"
        >编码</t-button
      >
      <t-button @click="decode" type="primary" :disabled="!state.inputStr"
        >解码</t-button
      >
    </div>
    <div class="split-content">
      <a-textarea
        class="full-textarea"
        v-model:value="state.resultStr"
        placeholder="编码/解码结果"
        readonly
      />
    </div>
  </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");
</style>
