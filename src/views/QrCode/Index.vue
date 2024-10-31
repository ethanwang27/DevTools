<script setup lang="ts">
import { reactive, ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { FormRules, FormInstance } from "element-plus";
import { writeText, writeImage } from "@tauri-apps/plugin-clipboard-manager";
import notify from "../../utils/notification";

const formRef = ref<FormInstance>();
const rules = reactive<FormRules>({
  qrCodeText: [
    {
      required: true,
      message: "二维码内容不能为空",
      trigger: "blur",
    },
  ],
});

const form = reactive({
  qrCodeText: "",
  imageFormat: "png",
  size: null,
  errorCorrection: 1,
});
const addDataUrlScheme = ref<Boolean>(false);
const boardRadius = {
  borderRadius: "var(--el-border-radius-large)",
  boxShadow: "var(--el-box-shadow-lighter)",
};

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
  notify("提示", "复制二维码Base64字符串成功！");
}

/**
 * 将二维码图片复制到剪贴板
 */
async function writeImageToClipboard() {
  if (!qrCodeBase64.value) return;
  const binaryString = atob(qrCodeBase64.value as string);
  const length = binaryString.length;
  const binaryArray = new Uint8Array(length);
  for (let i = 0; i < length; i++) {
    binaryArray[i] = binaryString.charCodeAt(i);
  }
  await writeImage(binaryArray);
  notify("提示", "复制二维码图片成功！");
}

onMounted(() => setDefaultFormData());

async function generateQrCode(formEl: FormInstance | undefined) {
  if (!formEl) return;
  await formEl.validate(async (valid) => {
    if (valid) {
      invoke("get_qr_code", {
        text: form.qrCodeText,
        size: form.size,
        errorCorrectionLevel: form.errorCorrection,
        imageFormat: form.imageFormat,
      })
        .then((res) => (qrCodeInfo.value = res))
        .catch((err) => console.log(err));
    }
  });
}
</script>
<template>
  <div class="action">
    <el-checkbox
      v-model="addDataUrlScheme"
      style="width: 11.5em"
      :disabled="!qrCodeInfo"
      >添加Data URI Scheme</el-checkbox
    >
    <el-button
      type="primary"
      :disabled="!qrCodeInfo"
      @click="writeBase64ToClipboard"
      >复制二维码Base64字符串</el-button
    >
    <el-button
      type="primary"
      class="button"
      :disabled="!qrCodeInfo"
      @click="writeImageToClipboard"
      >复制二维码</el-button
    >
    <el-button type="danger" class="button" @click="reset">重置</el-button>
    <el-button type="primary" @click="generateQrCode(formRef)" class="button"
      >生成</el-button
    >
  </div>
  <div class="content">
    <div class="content-item" :style="boardRadius">
      <el-scrollbar>
        <el-form
          :model="form"
          label-position="top"
          label-width="auto"
          :rules="rules"
          ref="formRef"
        >
          <el-form-item label="二维码内容" prop="qrCodeText">
            <el-input
              v-model="form.qrCodeText"
              type="textarea"
              placeholder="请输入二维码内容(url或文本信息)"
              @change="clearQrCode"
            />
          </el-form-item>
          <el-form-item label="文件格式" prop="fileType" @change="clearQrCode">
            <el-radio-group v-model="form.imageFormat">
              <el-radio value="png">PNG</el-radio>
              <el-radio value="jpeg">JPEG</el-radio>
            </el-radio-group>
          </el-form-item>

          <el-form-item
            label="纠错率"
            prop="errorCorrection"
            @change="clearQrCode"
          >
            <el-radio-group v-model="form.errorCorrection">
              <el-radio :value="0">7%</el-radio>
              <el-radio :value="1">15%</el-radio>
              <el-radio :value="2">25%</el-radio>
              <el-radio :value="3">30%</el-radio>
            </el-radio-group>
          </el-form-item>
          <el-form-item label="二维码大小" prop="size" @change="clearQrCode">
            <el-input-number v-model="form.size" :min="1" :max="40" />
          </el-form-item>
        </el-form>
      </el-scrollbar>
    </div>
    <div class="content-item" :style="boardRadius">
      <div class="qr-code">
        <el-image
          :src="qrCodeSrc()"
          fit="fill"
          v-if="qrCodeInfo"
          style="height: 100%; width: 100%"
        />
        <el-empty
          v-else
          description="请输入二维码信息后，点击【生成】按钮生成二维码！"
        />
      </div>
    </div>
  </div>
</template>
<style lang="less">
.action {
  display: flex;
  justify-content: right;
  height: 40px;
  margin: 0;
  padding: 0;

  .button {
    width: 80px;
  }
}

.content {
  display: flex;
  height: calc(100% - 65px);
  margin-top: 3px;

  .content-item {
    width: calc(50% -4px);
    height: 100%;
    flex: 1;
    padding: 1em;
    margin: 3px;

    .qr-code {
      margin: 1rem;
    }
  }
}
</style>
