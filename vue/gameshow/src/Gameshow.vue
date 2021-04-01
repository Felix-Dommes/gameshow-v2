<template>
  <div id="gameshow">
    <cookie-consent :lang="lang" @consent="got_consent" />
    
    <div class="mainWindow">
      <!-- side bar for player list and status etc -->
      <div class="sidebar">
        <template v-if="nickname != '' && lobby != ''">
          <transition name="transition" mode="out-in" appear>
            <div class="compWindow" style="text-align: center;">
              <span>{{ lang["Question"] }} {{ current_question.id }}</span>
            </div>
          </transition>
          
          <!--
          <transition name="transition" mode="out-in" appear>
            <player-list :players="players" :self="nickname" :question-type="current_question.type"></player-list>
          </transition>
          -->
        </template>
      </div>
      
      <!-- main stage for game stuff -->
      <div class="mainStage">
        <transition name="transition" mode="out-in" appear>
          <template v-if="selectedWindow == 'loading'">
            <div class="compWindow">
              {{ lang["Loading"] }}..
            </div>
          </template>
          
          <template v-else-if="selectedWindow == 'login-window'">
            <login-window :lang="lang" @set-name="set_name" />
          </template>
          
          <template v-else>
            <div class="compWindow">
              {{ lang["Waiting for players"] }}..
            </div>
          </template>
        </transition>
      </div>
    </div>
  </div>
</template>

<script>
import lang from './assets/lang.js'
import api from './assets/api.js'
import CookieConsent from './components/CookieConsent.vue'
import LoginWindow from './components/LoginWindow.vue'

export default
{
  name: 'Gameshow',
  components: {
    CookieConsent,
    LoginWindow,
  },
  data: () => { return {
    lang: lang.en,
    consent: false,
    selectedWindow: "loading",
    nickname: "",
    lobby: "",
    money: 1,
    jokers: 0,
    players: [],
    results_players_prev: [],
    results_players_new: [],
    current_question: {id: 0, type: "", category: "", question: "", answers: [], correct_answer: 0, wrong_answers: []},
    last_event_id: -1,
  }; },
  methods: {
    switchLanguage: function(language)
    {
      switch (language)
      {
        case "de":
          this.lang = lang.de;
          api.lang = lang.de;
          return true;
        case "en":
          this.lang = lang.en;
          api.lang = lang.en;
          return true;
        default:
          return false;
      }
    },
    got_consent: async function()
    {
      this.consent = true;
      //check if already logged in
      let name = await api.get_name();
      if (name != "") {
        this.nickname = name;
        //TODO
      }
      else {
        this.selectedWindow = "login-window";
      }
    },
    set_name: async function(name)
    {
      if (!this.consent) return;
      if (await api.set_name(name)) {
        this.nickname = name;
        //TODO
      }
    }
  },
  mounted: function()
  {
    //TODO check for lobby code in URL
  },
}
</script>

<style>
#gameshow
{
  margin: 0;
  padding: 0;
  font-family: 'Nunito Sans', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.mainWindow
{
  width: 100%;
  min-height: 100vh;
  overflow-x: hidden;
  background: radial-gradient(circle, #008800 0%, #002200 95%) no-repeat center fixed;
  display: flex;
  flex-wrap: wrap;
  flex-direction: row;
}

.sidebar
{
  flex: 1 0 auto;
  display: flex;
  flex-direction: column;
}

.mainStage
{
  flex: 1000 1 auto;
  min-height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
}

.compWindow
{
  border: 2px solid white;
  padding: 1em;
  background: radial-gradient(circle, #000080 0%, #000030 100%) no-repeat center;
  font-size: xx-large;
  color: white;
  text-shadow: 3px 2px 4px #222222;
}

@media (max-width: 50rem)
{
  .compWindow
  {
    font-size: large;
  }
}

.transition-enter, .transition-leave-to
{
  opacity: 0;
  transform: scale(0.5, 0.5) translate(0, 50%);
}
.transition-enter-active, .transition-leave-active, .transition-move
{
  transition: all 0.5s ease;
}
</style>
