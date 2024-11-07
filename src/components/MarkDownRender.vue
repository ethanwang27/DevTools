<!-- 渲染markdown -->
<script lang="ts" setup>
import markdownit from "markdown-it";
import hljs from "highlight.js";
import "github-markdown-css";
import "highlight.js/styles/github.css";
import { defineProps, computed } from "vue";
import MarkdownIt from "markdown-it";

const props = defineProps({
  text: { type: String, required: true },
});
const md: MarkdownIt = markdownit({
  html: true,
  linkify: true,
  typographer: true,
  langPrefix: "language-",
  highlight: function (str: any, lang: any) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return (
          '<pre><code class="hljs">' +
          hljs.highlight(str, { language: lang, ignoreIllegals: true }).value +
          "</code></pre>"
        );
      } catch (__) {}
    }

    return (
      '<pre><code class="hljs">' + md.utils.escapeHtml(str) + "</code></pre>"
    );
  },
});

const mdContent = computed(() => {
  return md.render(props.text);
});
</script>
<template>
  <div v-html="mdContent" class="markdown-body"></div>
</template>
<style lang="less"></style>
