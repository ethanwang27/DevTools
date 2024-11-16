import { createRouter, createMemoryHistory } from "vue-router";

export default createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: "/nothing",
      name: "nothing",
      component: () => import("./views/Nothing.vue"),
    },
    {
      path: "/generate-qr-code",
      name: "generateQrCode",
      component: () => import("./views/QrCode/Index.vue"),
    },
    {
      path: "/Unicode",
      name: "unicode",
      component: () => import("./views/Unicode/Index.vue"),
    },
    {
      path: "/preview-markdown",
      name: "previewMarkdown",
      component: () => import("./views/Markdown/PreviewMarkdown.vue"),
    },
  ],
});
