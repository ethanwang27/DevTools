<script setup lang="ts">
import { defineModel, defineProps } from "vue";
import { copyText } from "../utils/copyToClipboard";

const value = defineModel({ required: true, type: String });
const props = defineProps({
  isTextarea: {
    type: Boolean,
    default: false,
  },
  readonly: {
    type: Boolean,
    default: false,
  },
});
</script>
<template>
  <div class="input-compact">
    <a-input
      v-if="!props.isTextarea"
      v-model:value="value"
      :readonly="props.readonly"
    />
    <a-textarea
      v-if="props.isTextarea"
      v-model:value="value"
      :readonly="props.readonly"
    />
    <a-tooltip title="复制" color="blue">
      <a-button @click="copyText(value)" class="button" :disabled="!value">
        <template #icon><CopyTwoTone /></template>
      </a-button>
    </a-tooltip>
  </div>
</template>
<style lang="less" scoped>
.input-compact {
  display: flex;
}
.button {
  margin-left: 0.3rem;
}
</style>
