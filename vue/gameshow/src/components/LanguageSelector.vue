<template>
  <div class="window">
    <span class="flag" @click="set_lang('de')">🇩🇪</span>
    <span class="flag" @click="set_lang('en')">🇺🇸</span>
  </div>
</template>

<script>
import global from '../assets/global.js'

export default {
  name: 'LanguageSelector',
  methods: {
    set_lang: function(language)
    {
        document.cookie = "language=" + encodeURIComponent(language);
        this.$emit("set-lang", language);
    },
  },
  mounted: function()
  {
    let language = global.getCookie("language");
    if (language != "en") this.$emit("set-lang", language);
  },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.window
{
  z-index: 100;
  position: fixed;
  top: -2px;
  right: 0px;
  background-color: #ffffff;
  opacity: 0.5;
}

.window:hover
{
    opacity: 1;
}

.flag
{
  font-size: large;
  font-weight: bolder;
  margin: 0px 3px;
  cursor: pointer;
}
</style>
