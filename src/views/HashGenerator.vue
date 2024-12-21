<script setup lang="ts">
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";
import log from "../utils/logger";

interface IState {
  inputStr: String;
  generated: boolean;
  lowercase: boolean;
  MD2: string;
  MD4: string;
  MD5: string;
  SHA1: string;
  SHA224: string;
  SHA256: string;
  SHA384: string;
  SHA512: string;
}

const state = reactive<IState>({
  inputStr: "",
  generated: false,
  lowercase: true,
  MD2: "",
  MD4: "",
  MD5: "",
  SHA1: "",
  SHA224: "",
  SHA256: "",
  SHA384: "",
  SHA512: "",
});

/**
 * 生成Hash
 */
function generateHash() {
  if (!state.inputStr) {
    return;
  }
  /**
   * 调用rust api生成hash
   * @param hashType Hash类型
   * @param callback 生成Hash后的回调
   */
  const generateHash = async (
    hashType:
      | "MD2"
      | "MD4"
      | "MD5"
      | "SHA1"
      | "SHA224"
      | "SHA256"
      | "SHA384"
      | "SHA512",
    callback: (result: string) => {}
  ) => {
    let hash: string = await invoke("hash_generate", {
      data: state.inputStr,
      hashType: hashType,
      lowercase: state.lowercase,
    });
    callback(hash);
  };

  // 异步生成hash
  Promise.all([
    generateHash("MD2", (result) => (state.MD2 = result)),
    generateHash("MD4", (result) => (state.MD4 = result)),
    generateHash("MD5", (result) => (state.MD5 = result)),
    generateHash("SHA1", (result) => (state.SHA1 = result)),
    generateHash("SHA224", (result) => (state.SHA224 = result)),
    generateHash("SHA256", (result) => (state.SHA256 = result)),
    generateHash("SHA384", (result) => (state.SHA384 = result)),
    generateHash("SHA512", (result) => (state.SHA512 = result)),
  ])
    .then((_) => {
      message.info("生成成功");
    })
    .catch((err) => {
      log(`Hash生成失败: ${err}`, "error");
    });
}

/**
 * 将数据复制到剪贴板
 * @param data 复制到Clipboard的数据
 */
async function copyToClipboard(data: string) {
  if (!data) return;
  await writeText(data);
  message.info("复制成功");
}

/**
 * 清空输入框以及Hash结果
 */
function clear() {
  state.inputStr = "";
  clearHashResult();
}
/**
 * 清空Hash结果
 */
function clearHashResult() {
  state.MD2 = "";
  state.MD4 = "";
  state.MD5 = "";
  state.SHA1 = "";
  state.SHA224 = "";
  state.SHA224 = "";
  state.SHA256 = "";
  state.SHA384 = "";
  state.SHA512 = "";
}
</script>
<template>
  <div class="main-container">
    <a-textarea
      placeholder="请输入"
      v-model:value="state.inputStr"
      :rows="5"
      class="textarea"
      allow-clear
      show-count
      @change="clearHashResult"
    />
    <div class="action">
      <a-switch
        v-model:checked="state.lowercase"
        checked-children="小写"
        un-checked-children="大写"
        style="margin-top: 0.5rem"
        @change="generateHash"
      />
      <t-button @click="clear" type="primary" danger :disabled="!state.inputStr"
        >清空</t-button
      >
      <t-button @click="generateHash" type="primary" :disabled="!state.inputStr"
        >生成</t-button
      >
    </div>
    <div class="hash-content">
      <a-form :labelCol="{ style: 'width: 5rem' }" labelAlign="right">
        <a-form-item key="MD2" label="MD2">
          <a-input v-model:value="state.MD2" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.MD2"
                @click="copyToClipboard(state.MD2)"
              />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item key="MD4" label="MD4">
          <a-input v-model:value="state.MD4" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.MD4"
                @click="copyToClipboard(state.MD4)"
              />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item key="MD5" label="MD5">
          <a-input v-model:value="state.MD5" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.MD5"
                @click="copyToClipboard(state.MD5)"
              />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item key="SHA1" label="SHA1">
          <a-input v-model:value="state.SHA1" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.SHA1"
                @click="copyToClipboard(state.SHA1)"
              />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item key="SHA224" label="SHA224">
          <a-input v-model:value="state.SHA224" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.SHA224"
                @click="copyToClipboard(state.SHA224)"
              />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item key="SHA256" label="SHA256">
          <a-input v-model:value="state.SHA256" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.SHA256"
                @click="copyToClipboard(state.SHA256)"
              />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item key="SHA384" label="SHA384">
          <a-input v-model:value="state.SHA384" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.SHA384"
                @click="copyToClipboard(state.SHA384)"
              />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item key="SHA512" label="SHA512">
          <a-input v-model:value="state.SHA512" readonly>
            <template #suffix>
              <CopyTwoTone
                v-if="state.SHA512"
                @click="copyToClipboard(state.SHA512)"
              />
            </template>
          </a-input>
        </a-form-item>
      </a-form>
    </div>
  </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");
.main-container {
  height: 100%;

  .textarea {
    margin-bottom: 20px;
  }
}
</style>
