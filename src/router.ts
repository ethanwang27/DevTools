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
      path: "/test",
      name: "test",
      component: () => import("./views/Test.vue"),
    },
  ],
});
