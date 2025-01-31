import { createApp } from "vue";
import { createRouter, createWebHistory, RouterView } from "vue-router";
import Index from "./pages/index.vue";

import "./main.css";

const router = createRouter({
  history: createWebHistory(),
  routes: [{ path: "/", component: Index }],
});

createApp(RouterView).use(router).mount("body");
