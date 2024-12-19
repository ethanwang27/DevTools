<script setup lang="ts">
import { reactive } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import log from "../utils/logger";

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

function clear() {
  state.inputStr = "";
  state.resultStr = "";
}
function resultAsInput() {
  state.inputStr = state.resultStr;
  state.resultStr = "";
}
async function copyToClipboard() {
  if (!state.resultStr) return;
  await writeText(state.resultStr);
  message.info("复制成功");
}

function encoder() {
  if (!state.inputStr) {
    message.warn("请输入需要编码的文本");
    state.inputTextareaStatus = "error";
    return;
  }
  invoke("base64_encode", {
    data: state.inputStr,
  })
    .then((result) => {
      state.resultStr = result as string;
      message.info("编码成功");
    })
    .catch((err) => {
      message.error("编码失败");
      log(`base64编码失败: ${err}`, "error");
    });
}

function decoder() {
  if (!state.inputStr) {
    message.warn("请输入需要解码的文本");
    state.inputTextareaStatus = "error";
    return;
  }
  invoke("base64_decode", {
    data: state.inputStr,
  })
    .then((result) => {
      state.resultStr = result as string;
      message.info("解码成功");
    })
    .catch((err) => {
      message.error("解码失败");
      log(`base64解码失败: ${err}; 解码字符串：${state.inputStr}`, "error");
    });
}
</script>
<template>
  <div class="main-container vertical-tow-split-container">
    <div class="split-content">
      <a-textarea
        class="full-textarea"
        v-model:value="state.inputStr"
        :status="state.inputTextareaStatus"
        @change="
          () => {
            state.resultStr = '';
            state.inputTextareaStatus = '';
          }
        "
        placeholder="请输入需要编码或解码的文本"
      />
    </div>
    <div class="action">
      <t-button @click="clear" type="primary" danger>清空</t-button>
      <t-button
        @click="copyToClipboard"
        type="primary"
        :disabled="!state.resultStr"
        >复制</t-button
      >
      <t-button
        @click="resultAsInput"
        type="primary"
        :disabled="!state.resultStr"
        >结果作为输入</t-button
      >
      <t-button @click="encoder" type="primary" :disabled="!state.inputStr"
        >编码</t-button
      >
      <t-button @click="decoder" type="primary" :disabled="!state.inputStr"
        >解码</t-button
      >
    </div>
    <div class="split-content">
      <a-textarea
        class="full-textarea"
        v-model:value="state.resultStr"
        placeholder="编码或解码结果"
      />
    </div>
  </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");
</style>
