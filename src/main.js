import { createApp } from "vue";
import App from "./App.vue";
import router from './router/router'
import pinia from './store'
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

const app = createApp(App)
app.use(router)
  .use(pinia)
  .use(ElementPlus)
  .mount("#app");

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}