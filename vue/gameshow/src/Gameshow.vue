<template>
  <div id="gameshow">
    <CookieConsent :lang="lang" @consent="void(0)" />
    Content
  </div>
</template>

<script>
import lang from './assets/lang.js'
import CookieConsent from './components/CookieConsent.vue'

//const apiPath = "./api/";
//const eventPath = "./events/";

export default
{
  name: 'Gameshow',
  components: {
    CookieConsent,
  },
  data: () => { return {
    lang: lang.en,
  }; },
  methods: {
    switchLanguage: function(language)
    {
      switch (language)
      {
        case "de":
          this.lang = lang.de;
          return true;
        case "en":
          this.lang = lang.en;
          return true;
        default:
          return false;
      }
    },
  },
}
</script>

<style>
#gameshow
{
  font-family: 'Nunito Sans', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
