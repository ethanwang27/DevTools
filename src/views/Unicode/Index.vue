<script setup lang="ts">
import { ref, computed } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";
import TButton from "../../components/base/TButton.vue";

type TextareaStatus = "warn" | "error" | undefined;
const inputStr = ref<string>("");
const convertResult = ref<string>("");
const convertTextareaStatus = ref<TextareaStatus>(undefined);

/**
 * 字符编码转换
 * @param str 待转换的字符串
 * @param convertFunc 执行转换字符编码的回调函数
 */
function convert(str: string, convertFunc: () => string): void {
  new Promise(() => {
    if (str) {
      var result = convertFunc();
      console.log("转换结果", result);
      convertResult.value = result;
      message.success("转换成功");
    } else {
      message.error("待转换文本不能为空");
      convertTextareaStatus.value = "error";
    }
  });
}

function decodeUnicode(unicodeStr: string): string {
  console.log("unicode str", unicodeStr);
  let reg = /(\d|[a-f]){1,4}/gi;
  let result = unicodeStr
    .match(reg)
    ?.map((char) => {
      return String.fromCharCode(parseInt(char, 16));
    })
    .reduce((pre, next) => pre + next);

  console.log("unicode str", result);
  if (result === undefined) {
    message.error("unicode字符串不正确");
  }
  console.log(decodeURIComponent(result!));
  return decodeURIComponent(result!);
}
/**
 * Unicode转汉字
 */
function convertUnicodeToChinese() {
  convert(inputStr.value, () => {
    return decodeUnicode(inputStr.value);
  });
}

/**
 * 汉字转unicode
 */
function convertChineseToUnicode() {
  let convertFunc = () => {
    return Array.from({ length: inputStr.value.length }, (_, i) => i)
      .map((index) => {
        var charCode = inputStr.value.charCodeAt(index);
        return `\\u${charCode.toString(16).toUpperCase()}`;
      })
      .reduce((pre, next) => pre + next);
  };
  convert(inputStr.value, convertFunc);
}

/**
 * 复制转换结果
 */
function copyResult() {
  writeText(convertResult.value).then(() => message.success("复制成功"));
}

/**
 * 清空
 */
function clear() {
  inputStr.value = "";
  convertResult.value = "";
}

const copyDisable = computed(() => !convertResult.value);
</script>
<template>
  <div class="main-container vertical-two-split-container">
    <div class="split-content">
      <a-textarea
        placeholder="请输入unicode字符串或汉字"
        v-model:value="inputStr"
        :status="convertTextareaStatus"
        class="full-textarea"
        style="height: 100%"
        @change="() => (convertResult = '')"
      />
    </div>
    <div class="action">
      <t-button @click="clear" type="primary" danger :disabled="copyDisable"
        >清空</t-button
      >
      <t-button @click="copyResult" type="primary" :disabled="copyDisable"
        >复制结果</t-button
      >
      <t-button @click="convertUnicodeToChinese" type="primary"
        >Unicode转汉字</t-button
      >
      <t-button @click="convertChineseToUnicode" type="primary"
        >汉字转换Unicode</t-button
      >
    </div>
    <div class="split-content">
      <a-textarea
        placeholder="转换结果"
        v-model:value="convertResult"
        class="full-textarea"
        readOnly
      />
    </div>
  </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");
</style>
