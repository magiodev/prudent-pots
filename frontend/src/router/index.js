import {createRouter, createWebHistory} from "vue-router";

const routes = [
  {
    path: "/",
    name: "Prudent Pots",
    meta: {
      title: "Prudent Pots"
    },
    component: () => import("@/views/Home"),
  },
  {
    path: "/instructions",
    name: "Instructions",
    meta: {
      title: "Instructions",
    },
    component: () => import("@/views/Instructions"),
  },
  {
    path: "/end",
    name: "End",
    meta: {
      title: "End",
    },
    component: () => import("@/views/End"),
  },
  {
    path: "/:pathMatch(.*)*",
    name: "Uups!",
    meta: {
      title: "Uups!",
      error: 404,
    },
    component: () => import("@/views/Error"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
