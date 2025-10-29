import './main.scss';
import App from './App.vue';
import PrimeVue from 'primevue/config';
import { createApp } from 'vue';

const app = createApp(App);
app.use(PrimeVue);
