<!-- Url解析器 -->
<script setup lang="ts">
import { reactive, computed } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { message } from "ant-design-vue";

interface IState {
  inputUrl: string;
  protocol: string;
  host: string;
  port: string;
  path: string;
  originalPara: string;
  paraDisplayWay: ParaDisplayWay;
}

interface IUrlQueryPara {
  paraName: string;
  paraValue: string;
}

/** 查询参数的显示方式 */
enum ParaDisplayWay {
  Original,
  Json,
  Csv,
  Markdown,
}

const state = reactive<IState>({
  inputUrl: "",
  protocol: "",
  host: "",
  port: "",
  path: "",
  originalPara: "",
  paraDisplayWay: ParaDisplayWay.Json,
});

const queryParas = computed(() => {
  if (!state.originalPara) return [];
  return state.originalPara.split("&").map((item) => {
    let para = item.split("=");
    let result: IUrlQueryPara = {
      paraName: para[0],
      paraValue: para[1],
    };
    return result;
  });
});

const displayQueryParas = computed(() => {
  if (queryParas.value.length === 0) return "";
  if (state.paraDisplayWay === ParaDisplayWay.Original)
    return state.originalPara;
  if (state.paraDisplayWay === ParaDisplayWay.Json) {
    let text = queryParas.value
      .map((item) => `\t${item.paraName}: ${item.paraValue}`)
      .join(";\n");
    return `{\n${text}\n}`;
  }
  if (state.paraDisplayWay === ParaDisplayWay.Csv) {
    return queryParas.value
      .map((item) => `${item.paraName},${item.paraValue}`)
      .join("\n");
  }

  if (state.paraDisplayWay === ParaDisplayWay.Markdown) {
    let text = queryParas.value
      .map((item) => `|${item.paraName}|${item.paraValue}|`)
      .join("\n");
    return `|参数名|参数值|\n|---|---|\n${text}`;
  }
  return state.originalPara;
});

function clear() {
  state.inputUrl = "";
  clearResult();
}

function clearResult() {
  state.protocol = "";
  state.host = "";
  state.port = "";
  state.path = "";
  state.originalPara = "";
}
function parseUrl() {
  if (!state.inputUrl) return;
  // 匹配URL协议，开头字符串到 :// 结尾
  let protocol = /^\w+:\/\//.exec(state.inputUrl);
  state.protocol = protocol ? protocol[0].replace("://", "") : "";
  // 匹配URL主机，:// 到 / 之间的字符串
  let host = /:\/\/(.*?)(\/|:)/.exec(state.inputUrl);
  state.host = host
    ? host[0].replace("://", "").replace("/", "").replace(":", "")
    : "";
  // 匹配URL端口，: 到 / 之间的字符串
  let port = /:\d+\//.exec(state.inputUrl);
  state.port = port ? port[0].replace(":", "").replace("/", "") : "";
  // 匹配URL路径
  let path = /^(?:\w+:\/\/)?(?:[^\/:]+(?::\d+)?)(\/[^?]*)?/.exec(
    state.inputUrl
  );
  state.path = path && path[1] ? path[1] : "";
  // 匹配查询参数
  let parameters = /\?\S+/.exec(state.inputUrl);
  state.originalPara =
    parameters && parameters[0] ? parameters[0].replace("?", "") : "";
}

async function copyToClipboard(data: string) {
  if (!data) return;
  await writeText(data);
  message.info("复制成功");
}
</script>
<template>
  <div class="main-container">
    <div class="input-container">
      <a-textarea
        placeholder="请输入要解析的URL"
        class="full-textarea"
        v-model:value="state.inputUrl"
        @change="clearResult"
      />
    </div>
    <div class="action">
      <t-button @click="clear" type="primary" danger>清空</t-button>
      <t-button @click="parseUrl" type="primary" :disabled="!state.inputUrl"
        >解析</t-button
      >
    </div>
    <div class="result-container">
      <a-form
        class="form"
        :labelCol="{ style: 'width: 5rem' }"
        labelAlign="right"
      >
        <a-form-item label="协议" key="protocol">
          <div class="input-compact">
            <a-input v-model:value="state.protocol" readonly />
            <a-tooltip title="复制" color="blue">
              <a-button @click="copyToClipboard(state.protocol)">
                <template #icon><CopyTwoTone /></template>
              </a-button>
            </a-tooltip>
          </div>
        </a-form-item>
        <a-form-item label="Host" key="Host">
          <div class="input-compact">
            <a-input v-model:value="state.host" readonly />
            <a-tooltip title="复制" color="blue">
              <a-button @click="copyToClipboard(state.host)">
                <template #icon><CopyTwoTone /></template>
              </a-button>
            </a-tooltip>
          </div>
        </a-form-item>
        <a-form-item label="Port" key="port">
          <div class="input-compact">
            <a-input v-model:value="state.port" readonly />
            <a-tooltip title="复制" color="blue">
              <a-button @click="copyToClipboard(state.port)">
                <template #icon><CopyTwoTone /></template>
              </a-button>
            </a-tooltip>
          </div>
        </a-form-item>
        <a-form-item label="路径" key="path">
          <div class="input-compact">
            <a-input v-model:value="state.path" readonly />
            <a-tooltip title="复制" color="blue">
              <a-button @click="copyToClipboard(state.path)">
                <template #icon><CopyTwoTone /></template>
              </a-button>
            </a-tooltip>
          </div>
        </a-form-item>
        <a-form-item label="查询参数" class="query-parameters">
          <div style="display: flex; justify-content: flex-end">
            <a-radio-group v-model:value="state.paraDisplayWay">
              <a-radio-button :value="ParaDisplayWay.Original"
                >原始字符串</a-radio-button
              >
              <a-radio-button :value="ParaDisplayWay.Json">Json</a-radio-button>
              <a-radio-button :value="ParaDisplayWay.Csv">CSV</a-radio-button>
              <a-radio-button :value="ParaDisplayWay.Markdown"
                >Markdown</a-radio-button
              >
            </a-radio-group>
            <a-tooltip title="复制" color="blue">
              <a-button @click="copyToClipboard(displayQueryParas)">
                <template #icon>
                  <CopyTwoTone />
                </template>
              </a-button>
            </a-tooltip>
          </div>
        </a-form-item>
        <a-form-item style="margin-left: 5rem">
          <a-textarea v-model:value="displayQueryParas" readonly autosize />
        </a-form-item>
      </a-form>
    </div>
  </div>
</template>
<style lang="less" scoped>
@import url("/src//style/common.less");

.input-container {
  height: 5rem;
}
.result-container {
  height: calc(100% - 5rem - 3rem);
  display: flex;
  flex-direction: row;
  overflow-y: auto;

  .form {
    width: 100%;
    margin-right: 0.5rem;

    .input-compact {
      display: flex;
    }

    .query-parameters {
      margin-bottom: 0.3rem !important;
    }
  }
}
</style>
