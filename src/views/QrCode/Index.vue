<script setup lang="ts">
import { reactive, ref, computed, onMounted, UnwrapRef } from "vue";
import type { Rule } from "ant-design-vue/es/form";
import { invoke } from "@tauri-apps/api/core";
import { writeText, writeImage } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";
import TButton from "../../components/base/TButton.vue";

const formRef = ref();
const rules: Record<string, Rule[]> = {
  qrCodeText: [
    {
      required: true,
      message: "二维码内容不能为空",
      trigger: ["blur", "change"],
    },
  ],
};

interface FormState {
  qrCodeText: string;
  imageFormat: string;
  size: number | null;
  errorCorrection: number;
}

const form: UnwrapRef<FormState> = reactive({
  qrCodeText: "",
  imageFormat: "png",
  size: null,
  errorCorrection: 1,
});
const addDataUrlScheme = ref<Boolean>(false);

/**
 * 二维码信息
 */
const qrCodeInfo = ref<String | unknown>("");

/**
 * 二维码Base64字符串
 */
const qrCodeBase64 = computed(() => {
  if (!qrCodeInfo.value) return "";
  if (addDataUrlScheme.value) {
    return qrCodeSrc();
  } else {
    return qrCodeInfo.value;
  }
});

/**
 * 获取携带Data URI Scheme的二维码Base64字符串
 * 用于Img标签显示二维码
 */
const qrCodeSrc = () =>
  `data:image/${form.imageFormat};base64,${qrCodeInfo.value}`;

/**
 * 清空二维码信息
 */
function clearQrCode() {
  qrCodeInfo.value = undefined;
}

/**
 * 设置默认表单内容
 */
function setDefaultFormData() {
  form.qrCodeText = "";
  form.imageFormat = "png";
  form.size = null;
  form.errorCorrection = 1;
}

/**
 * 重置表单内容
 */
function reset() {
  setDefaultFormData();
  clearQrCode();
}

/**
 * 将二维码Base64字符串复制到剪贴板
 */
async function writeBase64ToClipboard() {
  if (!qrCodeBase64.value) return;
  await writeText(qrCodeBase64.value as string);
  message.success("已复制二维码Base64字符串");
}

/**
 * 将二维码图片复制到剪贴板
 */
async function writeImageToClipboard() {
  if (!qrCodeBase64.value) return;
  // 参考链接：https://stackoverflow.com/questions/76679845/save-canvas-data-as-png-or-jpg-in-tauri-using-fs-module
  const binaryString = atob(qrCodeBase64.value as string);
  const length = binaryString.length;
  const binaryArray = new Uint8Array(length);
  for (let i = 0; i < length; i++) {
    binaryArray[i] = binaryString.charCodeAt(i);
  }
  await writeImage(binaryArray);
  message.success("已复制二维码图片");
}

onMounted(() => setDefaultFormData());

async function generateQrCode() {
  await formRef.value.validate().then(() => {
    invoke("get_qr_code", {
      text: form.qrCodeText,
      size: form.size,
      errorCorrectionLevel: form.errorCorrection,
      imageFormat: form.imageFormat,
    })
      .then((res) => (qrCodeInfo.value = res))
      .catch((err) => console.log(err));
  });
}
</script>
<template>
  <div class="action">
    <a-space
      ><a-checkbox v-model:checked="addDataUrlScheme" :disabled="!qrCodeInfo"
        >添加Data URI Scheme</a-checkbox
      ></a-space
    >
    <t-button
      type="primary"
      :disabled="!qrCodeInfo"
      @click="writeBase64ToClipboard"
      >复制二维码Base64字符串</t-button
    >
    <t-button
      type="primary"
      :disabled="!qrCodeInfo"
      @click="writeImageToClipboard"
      >复制二维码</t-button
    >
    <t-button type="primary" danger @click="reset">重置</t-button>
    <t-button type="primary" @click="generateQrCode()">生成</t-button>
  </div>
  <div class="content">
    <a-card class="content-item ant-card-body">
      <div class="input-form">
        <a-form
          :model="form"
          layout="vertical"
          :rules="rules"
          ref="formRef"
          autocomplete="off"
        >
          <a-form-item label="二维码内容" name="qrCodeText">
            <a-textarea
              v-model:value="form.qrCodeText"
              placeholder="请输入二维码内容(url或文本信息)"
              :autosize="true"
              @change="clearQrCode"
            />
          </a-form-item>
          <a-form-item label="文件格式" name="fileType" @change="clearQrCode">
            <a-radio-group v-model:value="form.imageFormat">
              <a-radio value="png">PNG</a-radio>
              <a-radio value="jpeg">JPEG</a-radio>
            </a-radio-group>
          </a-form-item>

          <a-form-item
            label="纠错率"
            name="errorCorrection"
            @change="clearQrCode"
          >
            <a-radio-group v-model:value="form.errorCorrection">
              <a-radio :value="0">7%</a-radio>
              <a-radio :value="1">15%</a-radio>
              <a-radio :value="2">25%</a-radio>
              <a-radio :value="3">30%</a-radio>
            </a-radio-group>
          </a-form-item>
          <a-form-item label="二维码大小" name="size" @change="clearQrCode">
            <a-input-number v-model:value="form.size" :min="1" :max="40" />
          </a-form-item>
        </a-form>
      </div>
    </a-card>
    <a-card class="content-item">
      <div class="qr-code">
        <a-image
          :src="qrCodeSrc()"
          :alt="form.qrCodeText"
          :width="'100%'"
          v-if="qrCodeInfo"
          style="height: 100%; width: 100%"
        />
        <a-empty
          v-else
          description="请输入二维码信息后，点击【生成】按钮生成二维码！"
        />
      </div>
    </a-card>
  </div>
</template>
<style lang="less" scoped>
@import url("/src/style/common.less");

.content {
  display: flex;
  height: calc(100% - 50px);
  margin-top: 3px;

  .content-item {
    width: calc(50% -3px);
    height: 100%;
    flex: 1;

    .input-form {
      // margin: 3px 5px;
      padding: 3px 8px;
      height: 100%;
      overflow-y: auto;
    }

    .qr-code {
      margin: 3px 5px;
    }
  }
}
</style>
