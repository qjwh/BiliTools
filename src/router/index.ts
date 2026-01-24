import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import { HomeView } from '@/views/Home';
import { SettingsView } from '@/views/Settings';

export const ROUTES: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
  {
    path: '/settings',
    name: 'settings',
    component: SettingsView,
  },
] as const;

export default createRouter({
  history: createWebHistory(),
  routes: ROUTES,
});
