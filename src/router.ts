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
      path: '/Unicode',
      name: 'unicode',
      component: () => import('./views/Unicode/index.vue')
    },
    {
      path: "/test-markdown-render",
      name: "testMarkDown",
      component: () => import("./views/TestMarkdown.vue"),
    },
  ],
});
