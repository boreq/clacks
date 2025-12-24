import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import HomeView from '../views/HomeView.vue';
import PosterView from '../views/PosterView.vue';
import PosterViewWhite from '../views/PosterViewWhite.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
  {
    path: '/poster',
    name: 'poster',
    component: PosterView,
  },
  {
    path: '/poster-white',
    name: 'poster-white',
    component: PosterViewWhite,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
