import { createApp } from "vue";
import "./styles.css";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from "./App.vue";

var app = createApp(App)
app.use(ElementPlus)
app.mount("#app");

var block = () => {
    return false;
};
document.oncontextmenu = block;
document.onselectstart = block;
