import { createRouter, createWebHistory } from 'vue-router'
import Home from './views/Home.vue'
import Workbench from './views/Workbench.vue'
import Marker from './views/Marker.vue'
import Settings from './views/Settings.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/workbench',
    name: 'Workbench',
    component: Workbench
  },
  {
    path: '/marker',
    name: 'Marker',
    component: Marker
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router