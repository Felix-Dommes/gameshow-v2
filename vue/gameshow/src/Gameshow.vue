<template>
  <div id="gameshow">
    Content
    <CookieConsent @consent="void(0)" />
  </div>
</template>

<script>
import CookieConsent from './components/CookieConsent.vue'

export default
{
  name: 'Gameshow',
  components: {
    CookieConsent,
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
