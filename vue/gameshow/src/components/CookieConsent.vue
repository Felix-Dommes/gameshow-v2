<template>
  <div class="background" v-if="visible">
    <div class="window">
      <p class="text">{{ lang["This site uses (only functional) cookies!"] }}</p>
      <div class="button-accept" @click="accept">{{ lang["Accept"] }}</div>
    </div>
  </div>
</template>

<script>
import global from '../assets/global.js'

export default {
  name: 'CookieConsent',
  props: ["lang"],
  data: function() { return {
    visible: false,
  }; },
  methods: {
    accept: function()
    {
      document.cookie = "CONSENT=1";
      this.visible = false;
      this.$emit("consent");
    },
    show : function()
    {
      this.visible = true;
    }
  },
  mounted: function()
  {
    let consent = global.getCookie("CONSENT");
    if (consent != "1") this.show();
    else this.$emit("consent");
  },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.background
{
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.window
{
  background-color: #ffffff;
  padding: 1rem;
}

.text
{
  margin-top: 0;
  margin-bottom: 1rem;
  font-size: large;
  font-weight: bolder;
}

.button-accept
{
  float: right;
  background-color: #00a732;
  border: 1px solid transparent;
  border-color: #017223;
  border-radius: 4px;
  cursor: pointer;
  padding: 3px 10px;
  text-align: center;
  color: #c3ffd5;
  font-size: large;
}
.button-accept:hover
{
  background-color: #00962d;
}
.button-accept:focus
{
  border-color: #000000;
}
.button-accept:active
{
  background-color: #01882a;
}
</style>
