<script setup lang="ts">
import { ref, computed } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";

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
  let reg = /(\d|[a-f]){4}/gi;
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
  <div class="top-input">
    <a-textarea
      placeholder="请输入unicode字符串或汉字"
      v-model:value="inputStr"
      :status="convertTextareaStatus"
      class="textarea"
      style="height: 100%"
      @change="() => (convertResult = '')"
    />
  </div>
  <div class="center-button">
    <a-button
      @click="convertUnicodeToChinese"
      class="button"
      type="primary"
      ghost
      >Unicode转汉字</a-button
    >
    <a-button
      @click="convertChineseToUnicode"
      class="button"
      type="primary"
      ghost
      >汉字转换Unicode</a-button
    >
    <a-button
      @click="copyResult"
      class="button"
      type="primary"
      ghost
      :disabled="copyDisable"
      >复制结果</a-button
    >
    <a-button @click="clear" class="button" type="primary" ghost danger
      >清空</a-button
    >
  </div>
  <div class="bottom-input">
    <a-textarea
      placeholder="转换结果"
      v-model:value="convertResult"
      class="textarea"
      readOnly
    />
  </div>
</template>
<style lang="less">
@input-height: calc((100% - 46px) / 2);
@input-margin: 0px 5px 0px 0px;

.top-input {
  height: @input-height;
  margin: @input-margin;
}

.center-button {
  display: flex;
  justify-content: right;
  height: 40px;
  .button {
    margin: 3px 5px;
  }
}

.bottom-input {
  height: @input-height;
  margin: @input-margin;
}
.textarea {
  height: 100% !important;
  resize: none;
}
</style>
