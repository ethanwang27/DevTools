<script setup lang="ts">
import { EHashType } from "../enums/HashType";
import { reactive, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import { Rule } from "ant-design-vue/es/form";
import InputWithCopy from "../components/InputWithCopy.vue";

interface IState {
  key: string;
  data: string;
  hashType: EHashType;
  result: IHmacResult;
}

interface IHmacResult {
  hex: string;
  base64: string;
}

const state = reactive<IState>({
  key: "",
  data: "",
  hashType: EHashType.SHA512,
  result: {
    hex: "",
    base64: "",
  },
});

const inputFormRef = ref();

const formRules: Record<string, Rule[]> = {
  key: [{ required: true, message: "请输入密钥", trigger: "blur" }],
  data: [{ required: true, message: "请输入加密内容", trigger: "blur" }],
};

function clear() {
  setDefaultState();
  inputFormRef.value.clearValidate();
  message.info("清空成功");
}

function clearResult() {
  inputFormRef.value.clearValidate();
  state.result = {
    hex: "",
    base64: "",
  };
}

function setDefaultState() {
  state.data = "";
  state.key = "";
  state.hashType = EHashType.SHA512;
  state.result.hex = "";
  state.result.base64 = "";
}

function generateHmac() {
  inputFormRef.value.validate().then(() => {
    invoke("generate_hmac", {
      key: state.key,
      data: state.data,
      hashType: EHashType[state.hashType],
    })
      .then((result) => {
        state.result = result as IHmacResult;
      })
      .catch((err) => {
        message.error(`Hmac生成失败: ${err}`);
      });
  });
}

onMounted(() => setDefaultState());
</script>
<template>
  <div class="main-container container">
    <a-form
      ref="inputFormRef"
      :rules="formRules"
      :model="state"
      :labelCol="{ style: 'width: 6rem' }"
    >
      <a-form-item name="hash" label="哈希算法">
        <a-radio-group v-model:value="state.hashType" @change="clearResult">
          <a-radio :value="EHashType.MD2">MD2</a-radio>
          <a-radio :value="EHashType.MD4">MD4</a-radio>
          <a-radio :value="EHashType.MD5">MD5</a-radio>
          <a-radio :value="EHashType.SHA1">SHA1</a-radio>
          <a-radio :value="EHashType.SHA224">SHA224</a-radio>
          <a-radio :value="EHashType.SHA256">SHA256</a-radio>
          <a-radio :value="EHashType.SHA384">SHA384</a-radio>
          <a-radio :value="EHashType.SHA512">SHA512</a-radio>
        </a-radio-group>
      </a-form-item>
      <a-form-item name="key" label="密钥">
        <a-input v-model:value="state.key" @change="clearResult" />
      </a-form-item>
      <a-form-item name="data" label="加密内容">
        <a-textarea
          v-model:value="state.data"
          @change="clearResult"
          :rows="10"
        />
      </a-form-item>
      <a-form-item class="action">
        <t-button @click="clear" type="primary" danger>重置</t-button>
        <t-button @click="generateHmac" type="primary">生成</t-button>
      </a-form-item>

      <a-form-item name="hex" label="结果(hex)">
        <input-with-copy
          v-model="state.result.hex"
          :isTextarea="true"
          :readonly="true"
        />
      </a-form-item>
      <a-form-item name="base64" label="结果(base64)">
        <input-with-copy
          v-model="state.result.base64"
          :isTextarea="true"
          :readonly="true"
        />
      </a-form-item>
    </a-form>
  </div>
</template>
<style lang="less" scoped>
@import url("/src//style/common.less");

.container {
  padding: 1rem;
  overflow-y: auto;
}

.input-compact {
  display: flex;
}
</style>
