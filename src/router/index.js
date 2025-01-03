import { createRouter, createWebHistory } from "vue-router";
import Config from "../views/Config.vue";

const routes = [
  {
    path: "/",
    name: "schedule",
    component: () => import("../views/Schedule.vue"),
  },
  {
    path: "/config",
    name: "config",
    component: Config,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
