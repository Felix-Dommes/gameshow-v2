<template>
  <div id="gameshow">
    <cookie-consent :lang="lang" @consent="got_consent" />
    
    <div class="mainWindow">
      <!-- side bar for player list and status etc -->
      <div class="sidebar">
        <!-- TODO language switch -->
        
        <template v-if="nickname != '' && lobby != ''">
          <transition name="transition" mode="out-in" appear>
            <div class="compWindow" style="text-align: center;">
              <span>{{ lang["Question"] }} {{ current_question.id }}</span>
            </div>
          </transition>
          
          <!-- TODO
          <transition name="transition" mode="out-in" appear>
            <player-list :lang="lang" :players="players" :self="nickname" :question-type="current_question.type"></player-list>
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
          
          <template v-else-if="selectedWindow == 'lobby-selection'">
            <lobby-selection :lang="lang" :not-found="param_not_found" @create-lobby="create_lobby" @join-lobby="join_lobby" />
          </template>
          
          <template v-else-if="selectedWindow == 'lobby-menu'">
            <lobby-menu :lang="lang" :admin="my_uuid == admin" :lobby_id="lobby" :question_sets="question_sets" @start-game="start_game" />
          </template>
          
          <!-- TODO
            ...
          -->
          
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
import global from './assets/global.js'
import lang from './assets/lang.js'
import api from './assets/api.js'
import CookieConsent from './components/CookieConsent.vue'
import LoginWindow from './components/LoginWindow.vue'
import LobbySelection from './components/LobbySelection.vue'
import LobbyMenu from './components/LobbyMenu.vue'

export default
{
  name: 'Gameshow',
  components: {
    CookieConsent,
    LoginWindow,
    LobbySelection,
    LobbyMenu,
  },
  data: () => { return {
    lang: lang.en,
    question_sets: [],
    
    consent: false,
    selectedWindow: "loading",
    my_uuid: "",
    nickname: "",
    lobby: "",
    admin: "",
    admin_plays: true, //only local, don't use on non-admin clients
    money: 1,
    jokers: 0,
    players: [],
    results_players_prev: [],
    results_players_new: [],
    current_question: {id: 0, type: "", category: "", question: "", answers: [], correct_answer: 0, wrong_answers: []},
    last_event_id: -1, //TODO process events only in order: no > last_event_id, but = last_event_id + 1
    
    param_not_found: false,
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
      let [name, uuid] = await api.get_name();
      if (name != "" && uuid != "") {
        this.my_uuid = uuid;
        this.nickname = name;
        let lobby_id = global.extract_lobby_id();
        if (lobby_id != "")
        {
          if (!await this.join_lobby(lobby_id)) this.selectedWindow = "lobby-selection";
        }
        else
        {
          this.selectedWindow = "lobby-selection";
        }
      }
      else {
        this.selectedWindow = "login-window";
      }
    },
    set_name: async function(name)
    {
      if (!this.consent) return;
      const uuid = await api.set_name(name);
      if (uuid != "") {
        this.my_uuid = uuid;
        this.nickname = name;
        let lobby_id = global.extract_lobby_id();
        if (lobby_id != "")
        {
          if (!await this.join_lobby(lobby_id)) this.selectedWindow = "lobby-selection";
        }
        else
        {
          this.selectedWindow = "lobby-selection";
        }
      }
    },
    create_lobby: async function()
    {
      if (!this.consent) return;
      let result = await api.create_lobby();
      if (!result.valid) return;
      let lobby_id = result.lobby_id;
      result = await api.join_lobby(lobby_id);
      this.param_not_found = result.not_found;
      if (!result.valid) return;
      this.lobby = lobby_id;
      this.admin = result.admin;
      this.nickname = result.new_name;
      //TODO: dont always join?; connect to event stream and retrieve previous events
      window.history.pushState("lobby", "Gameshow Lobby", "#" + lobby_id);
      this.selectedWindow = "lobby-menu";
    },
    join_lobby: async function(lobby_id)
    {
      if (!this.consent) return false;
      if (lobby_id == "") return false;
      let result = await api.join_lobby(lobby_id);
      this.param_not_found = result.not_found;
      if (!result.valid) return false;
      this.lobby = lobby_id;
      this.admin = result.admin;
      this.nickname = result.new_name;
      //TODO: connect to event stream and retrieve previous events
      window.history.pushState("lobby", "Gameshow Lobby", "#" + lobby_id);
      this.selectedWindow = "lobby-menu";
      return true;
    },
    start_game: async function(admin_plays)
    {
      this.admin_plays = admin_plays;
      if (!admin_plays && this.admin == this.my_uuid)
      {
        await api.leave_lobby(this.lobby);
      }
      
      //TODO start game
    },
  },
  mounted: async function()
  {
    this.question_sets = await api.get_question_sets();
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
