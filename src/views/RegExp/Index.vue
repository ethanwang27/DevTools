<script setup lang="ts">
import { reactive, computed, ref } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";
import { throttle } from "../../utils/common.js";

interface IState {
  regExp: string;
  isValidRegExp: boolean;
  rawText: string;
  originText: string;
  matchTexts: Array<IMatchResult>;
  highlightText: string;
}

interface IMatchResult {
  text: string;
  start: number;
  end: number;
}

const highlightRef = ref(null);
const state = reactive<IState>({
  rawText: "",
  originText: "",
  isValidRegExp: true,
  regExp: "",
  matchTexts: [],
  highlightText: "",
});

/**正则表达式输入框状态（非正则表达式时为error） */
const inputStatus = computed(() => (state.isValidRegExp ? "" : "error"));

/**
 * 是否合法的正则表达式，若是则返回Regexp，否则返回undefined
 */
function isRegexp(): RegExp | undefined {
  try {
    return new RegExp(state.regExp, "g");
  } catch {
    return undefined;
  }
}

/** 重置匹配结果 */
function resetMatch() {
  state.isValidRegExp = true;
  state.highlightText = state.rawText;
  state.matchTexts = [];
}

/**
 * 匹配正则表达式
 */
function doRegexpMatch(event) {
  resetMatch();
  if (!state.regExp) {
    return;
  }
  const reg = isRegexp();

  if (reg === undefined) {
    state.isValidRegExp = false;
    return;
  }
  if (!state.rawText) {
    return;
  }

  state.highlightText = state.rawText
    .replace(/\n$/g, "\n\n") // 输入框文本末行为换行符时，white-space: pre-wrap会将换行挂起，因此多添加一个换行避免高亮无法对齐
    .replace(reg, (text, start) => {
      state.matchTexts.push({
        text: text,
        start: start,
        end: start + text.length,
      });
      return `<span class="matched-text">${text}</span>`;
    });
}

/** 清空输入内容以及匹配结果 */
function clear() {
  state.regExp = "";
  state.rawText = "";
  state.matchTexts = [];
  state.isValidRegExp = true;
  state.highlightText = "";
}

/** 匹配结构保存到剪贴板 */
function saveToClipboard() {
  let copyInfo = state.matchTexts.map((item) => item.text).join("\n");
  writeText(copyInfo).then(() => message.success("复制成功"));
}

function onScroll(event) {
  if (highlightRef.value) {
    highlightRef.value.scrollTop = event.target.scrollTop;
  }
}
</script>
<template>
  <div class="main-container">
    <div class="action">
      <t-button
        type="primary"
        @click="saveToClipboard"
        :disabled="state.matchTexts.length === 0"
        >复制结果</t-button
      >
      <t-button
        type="primary"
        danger
        @click="clear"
        :disabled="!state.rawText && !state.regExp"
        >清空</t-button
      >
    </div>
    <div class="input-regexp">
      <a-input
        v-model:value="state.regExp"
        placeholder="请输入正则表达式"
        style="width: 100%"
        :status="inputStatus"
        @change="doRegexpMatch"
      />
    </div>
    <div class="match-content">
      <div ref="highlightRef" v-html="state.highlightText" class="text-div" />
      <div class="text-wrap">
        <a-textarea
          class="full-textarea"
          v-model:value="state.rawText"
          @change="doRegexpMatch"
          @scroll="onScroll"
          placeholder="请输入需要匹配的文本"
        />
      </div>
    </div>
    <div class="match-result">
      <a-list v-for="(item, index) in state.matchTexts" :key="index"
        >{{ item.text }}
        <span class="result-range"
          >[{{ item.start }}:{{ item.end }}]</span
        ></a-list
      >
    </div>
  </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");
@import url("/src/style/variables.less");

@header-height: calc(6px + 40px + 50px);

.input-regexp {
  height: 50px;
  margin: 0;
  padding: @common-padding;
}

.match-content {
  height: calc((100% - @header-height) * 0.6 - 6px);
  margin: @common-margin;
  position: relative;
  :deep(.matched-text) {
    background-color: rgb(41, 187, 246);
    position: relative;
    // 确保浮动在页面最上方
    z-index: 100000;
    word-break: break-all;
    white-space: pre-wrap;
  }

  .text-div {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 4px 11px;
    white-space: pre-wrap;
    word-wrap: break-word;
    color: rgba(0, 0, 0, 0.85);
    line-height: 22px;
    transition: all 0.3s height 0s;
    border: 1px solid #fff;
    border-radius: 2px;
    overflow: auto;
    text-align: left;
  }

  .text-wrap {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    position: absolute;
    z-index: 1;
    top: 0;
    left: 0;
  }
}

.match-result {
  height: calc((100% - @header-height) * 0.4 - 6px);
  margin: @common-margin;
  padding: @common-padding;
  overflow: auto;
  .result-range {
    font-size: 0.9rem;
    color: lightgray;
  }
}
</style>
