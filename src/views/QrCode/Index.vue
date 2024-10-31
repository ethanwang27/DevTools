<script setup lang="ts">
import { reactive, ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { FormRules, FormInstance } from "element-plus";

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

const qrCodeInfo = ref<String | unknown>("");
const qrCodeBase64 = computed(() => {
  if (!qrCodeInfo.value) return "";
  if (addDataUrlScheme.value) {
    return qrCodeSrc();
  } else {
    return qrCodeInfo.value;
  }
});

const qrCodeSrc = () =>
  `data:image/${form.imageFormat};base64,${qrCodeInfo.value}`;

function clearQrCode() {
  qrCodeInfo.value = undefined;
}

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
  <div>
    <el-form
      :model="form"
      label-position="right"
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
      <el-row>
        <el-col :span="6">
          <el-form-item label="文件格式" prop="fileType" @change="clearQrCode">
            <el-radio-group v-model="form.imageFormat">
              <el-radio value="png">PNG</el-radio>
              <el-radio value="jpeg">JPEG</el-radio>
            </el-radio-group>
          </el-form-item>
        </el-col>
        <el-col :span="6">
          <el-form-item label="二维码大小" prop="size" @change="clearQrCode">
            <el-input-number v-model="form.size" :min="1" :max="40" />
          </el-form-item>
        </el-col>
        <el-col :span="10">
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
        </el-col>
      </el-row>
    </el-form>
  </div>
  <div class="action">
    <el-button type="primary" @click="generateQrCode(formRef)" class="button"
      >生成</el-button
    >
  </div>
  <div class="bottom">
    <div class="qr-code">
      <el-input
        type="textarea"
        v-model="qrCodeBase64"
        style="height: calc(100% - 3rem)"
      />
      <div>
        <label>二维码图片Base64字符串</label>
        <el-checkbox v-model="addDataUrlScheme" style="margin-left: 10px"
          >添加Data URI Scheme</el-checkbox
        >
      </div>
    </div>
    <div class="qr-code">
      <el-image
        :src="qrCodeSrc()"
        fit="fit"
        v-if="qrCodeInfo"
        style="height: calc(100% - 3rem); width: 100%"
      />
      <el-empty
        v-else
        description="请输入二维码信息后，点击【生成】按钮生成二维码！"
        style="height: calc(100% - 3rem)"
      />
    </div>
  </div>
</template>
<style>
.bottom {
  display: flex;
  height: calc(100% - 160px);
}
.qr-code {
  flex: 1;
  width: 50%;
  height: 100%;
  margin: 1rem;
}
.action {
  display: flex;
  justify-content: right;
}
.button {
  width: 80px;
}
/* el-input高度100% */
.el-textarea__inner {
  height: 100%;
}
</style>
