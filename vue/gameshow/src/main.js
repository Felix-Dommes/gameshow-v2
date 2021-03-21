import Vue from 'vue'
import Gameshow from './Gameshow.vue'

Vue.config.productionTip = false

new Vue({
  render: h => h(Gameshow),
}).$mount('#app')
