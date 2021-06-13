<template>
  <div id="gameshow">
    <language-selector @set-lang="switchLanguage" />
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
            <lobby-menu :lang="lang" :admin="my_uuid == admin" :lobby_id="lobby" :question_sets="question_sets" :sync_params="lobby_menu_params" @start-game="start_game" />
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

import LanguageSelector from './components/LanguageSelector.vue'
import CookieConsent from './components/CookieConsent.vue'
import LoginWindow from './components/LoginWindow.vue'
import LobbySelection from './components/LobbySelection.vue'
import LobbyMenu from './components/LobbyMenu.vue'

export default
{
  name: 'Gameshow',
  components: {
    LanguageSelector,
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
    animation_in_progress: false,
    
    event_stream: null,
    last_event_id: -1,
    event_queue: [],
    
    param_not_found: false,
    lobby_menu_params: {
      lobby_open: true,
      initial_money: "500",
      initial_jokers: "3",
      normal_q_money: "500",
      estimation_q_money: "1000",
      question_set: ""
    },
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
      await this.setup_event_listener();
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
      await this.setup_event_listener();
      window.history.pushState("lobby", "Gameshow Lobby", "#" + lobby_id);
      this.selectedWindow = "lobby-menu";
      return true;
    },
    setup_event_listener: async function()
    {
      this.event_stream = await api.get_event_stream(this.lobby);
      this.event_stream.addEventListener("ping", () => { this.handle_event_queue(); });
      this.event_stream.addEventListener("game_event", (event) => {
        this.handle_event(JSON.parse(event.data));
      });
      this.event_queue = await api.get_events(this.lobby);
      this.handle_event_queue();
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
    
    waitForPlayers: function()
    {
      this.selectedWindow = "";
    },
    finishedAnimation: function()
    {
      this.animation_in_progress = false;
    },
    
    handle_event_queue: function()
    {
      this.event_queue.sort( (a, b) => a.id - b.id );
      this.event_queue.filter((event) => {
        if (event.id <= this.last_event_id) return false;
        else if (event.id == this.last_event_id + 1)
        {
          this.handle_event(event);
          return false;
        }
        else return true;
      });
    },
    handle_new_event: function(event)
    {
      //handle queueing
      this.handle_event_queue();
      if (event.id > this.last_event_id + 1) this.event_queue.push(event);
      else if (event.id == this.last_event_id + 1) this.handle_event(event);
    },
    handle_event: function(event)
    {
      switch (event.event_name)
      {
        case "BeginNormalQAnswering":
          this.eventBeginNormalQAnswering(event.event[event.event_name]);
          break;
        case "BeginBettingQBetting":
          this.eventBeginBettingQBetting(event.event[event.event_name]);
          break;
        case "BeginBettingQAnswering":
          this.eventBeginBettingQAnswering(event.event[event.event_name]);
          break;
        case "BeginEstimationQAnswering":
          this.eventBeginEstimationQAnswering(event.event[event.event_name]);
          break;
        case "BeginVersusQSelecting":
          this.eventBeginVersusQSelecting(event.event[event.event_name]);
          break;
        case "BeginVersusQAnswering":
          this.eventBeginVersusQAnswering(event.event[event.event_name]);
          break;
        case "ShowResults":
          this.eventShowResults(event.event[event.event_name]);
          break;
        case "GameEnding":
          this.eventGameEnding(event.event[event.event_name]);
          break;
        case "BackToMenu":
          this.eventBackToMenu(event.event[event.event_name]);
          break;
        case "PlayerListUpdate":
          this.eventPlayerListUpdate(event.event[event.event_name]);
          break;
        case "LobbySettingsUpdate":
          this.eventLobbySettingsUpdate(event.event[event.event_name]);
          break;
        default:
          console.log("Unknown event: " + event.event_name);
      }
      this.last_event_id = event.id;
    },
    eventBeginNormalQAnswering: function(event)
    {
      this.animation_in_progress = false;
      this.current_question.id = event.current_question;
      this.current_question.type = event.question_type;
      this.current_question.category = event.category;
      this.current_question.question = event.question;
      this.current_question.answers = event.answers;
      this.current_question.correct_answer = 0;
      this.current_question.wrong_answers = [];
      this.selectedWindow = "question-asker";
    },
    eventBeginBettingQBetting: function(event)
    {
      this.animation_in_progress = false;
      this.current_question.id = event.current_question;
      this.current_question.type = event.question_type;
      this.current_question.category = event.category;
      this.current_question.question = "";
      this.current_question.answers = [];
      this.current_question.correct_answer = 0;
      this.current_question.wrong_answers = [];
      this.selectedWindow = "question-category-betting";
    },
    eventBeginBettingQAnswering: function(event)
    {
      this.current_question.question = event.question;
      this.current_question.answers = event.answers;
      this.selectedWindow = "question-asker";
    },
    eventBeginEstimationQAnswering: function(event)
    {
      this.animation_in_progress = false;
      this.current_question.id = event.current_question;
      this.current_question.type = event.question_type;
      this.current_question.category = event.category;
      this.current_question.question = event.question;
      this.current_question.answers = [];
      this.current_question.correct_answer = 0;
      this.current_question.wrong_answers = [];
      this.selectedWindow = "question-estimator";
    },
    eventBeginVersusQSelecting: function(event)
    {
      this.animation_in_progress = false;
      this.current_question.id = event.current_question;
      this.current_question.type = event.question_type;
      this.current_question.category = event.category;
      this.current_question.question = "";
      this.current_question.answers = [];
      this.current_question.correct_answer = 0;
      this.current_question.wrong_answers = [];
      this.selectedWindow = "question-vs-attacker";
    },
    eventBeginVersusQAnswering: function(event)
    {
      this.current_question.question = event.question;
      this.current_question.answers = event.answers;
      this.selectedWindow = "question-asker";
    },
    eventShowResults: function(event)
    {
      this.current_question.correct_answer = event.correct_answer;
      this.results_players_prev = event.previous_player_data;
      this.results_players_new = event.player_data;
      
      this.animation_in_progress = true;
      setTimeout(function(comp) { comp.finishedAnimation(); }, 15000, this);
      this.selectedWindow = "result-display";
    },
    eventGameEnding: function(event)
    {
      this.animation_in_progress = false;
      this.current_question.type = "";
      
      this.results_players_new = event.player_data;
      this.selectedWindow = "game-end-screen";
    },
    eventBackToMenu: function(event)
    {
      this.animation_in_progress = false;
      this.current_question.type = "";
      
      this.lobby_menu_params.open = event.open;
      this.selectedWindow = "lobby-menu";
    },
    eventPlayerListUpdate: function(event)
    {
      //TODO ? if (!this.animation_in_progress)
      this.players = event.player_data;
      let found_myself = false;
      for (var player of this.players)
      {
        if (player.uuid == this.my_uuid)
        {
          this.money = player.money;
          this.jokers = player.jokers;
          found_myself = true;
        }
      }
      if (!found_myself)
      { //player was kicked or server restarted or so, have to reload!
        alert(this.lang["Something went wrong! Reloading page.."]);
        window.location.href = "/";
      }
    },
    eventLobbySettingsUpdate: function(event)
    {
      this.lobby_menu_params = {
        lobby_open: event.open,
        initial_money: event.initial_money,
        initial_jokers: event.initial_jokers,
        normal_q_money: event.normal_q_money,
        estimation_q_money: event.estimation_q_money,
        question_set: event.question_set
      };
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
