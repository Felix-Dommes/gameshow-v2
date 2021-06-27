<template>
  <div id="gameshow">
    <language-selector :consent="consent" @set-lang="switchLanguage" />
    <cookie-consent :lang="lang" @consent="got_consent" />
    
    <div class="mainWindow">
      <!-- side bar for player list and status etc -->
      <div class="sidebar">
        <template v-if="nickname != '' && lobby != ''">
          <transition name="transition" mode="out-in" appear>
            <div class="compWindow" style="text-align: center;" key="question">
              <span>{{ lang["Question"] }} {{ current_question.id }}</span>
              <next-button v-if="nickname == admin" :lang="lang" :lobby_id="lobby" :selected_window="selectedWindow" />
            </div>
          </transition>
          
          <transition name="transition" mode="out-in" appear>
            <player-list :lang="lang" :players="players" :self="nickname" :admin="admin" :lobby_id="lobby" :question_type="current_question.type" @admin-leaves="admin_left" key="player-list" />
          </transition>
        </template>
      </div>
      
      <!-- main stage for game stuff -->
      <div class="mainStage">
        <transition name="transition" mode="out-in" appear>
          <template v-if="selectedWindow == 'loading'">
            <div class="compWindow" key="loading">
              {{ lang["Loading"] }}..
            </div>
          </template>
          
          <template v-else-if="selectedWindow == 'login-window'">
            <login-window :lang="lang" @set-name="set_name" key="login-window" />
          </template>
          
          <template v-else-if="selectedWindow == 'lobby-selection'">
            <lobby-selection :lang="lang" :join_errors="lobby_selection_params" @create-lobby="create_lobby" @join-lobby="join_lobby" key="lobby-selection" />
          </template>
          
          <template v-else-if="selectedWindow == 'lobby-menu'">
            <lobby-menu :lang="lang" :admin="nickname == admin" :lobby_id="lobby" :question_sets="question_sets" :sync_params="lobby_menu_params" @start-game="start_game" key="lobby-menu" />
          </template>
          
          <template v-else-if="selectedWindow == 'question-category-betting'">
            <question-category-betting :lang="lang" :watch_only="watch_only" :question="current_question" :max_bet="money" @bet-money="bet_money" key="question-category-betting" />
          </template>
          
          <template v-else-if="selectedWindow == 'question-vs-attacker'">
            <question-vs-attacker :lang="lang" :watch_only="watch_only" :question="current_question" :players="players" :self="nickname" @attack-player="attack_player" key="question-vs-attacker" />
          </template>
          
          <template v-else-if="selectedWindow == 'question-asker'">
            <question-asker :lang="lang" :watch_only="watch_only" :question="current_question" :joker-available="jokers > 0" @joker="get_joker" @answered="select_answer" key="question-asker" />
          </template>
          
          <template v-else-if="selectedWindow == 'question-estimator'">
            <question-estimator :lang="lang" :watch_only="watch_only" :question="current_question" @answered="select_answer" key="question-estimator" />
          </template>
          
          <template v-else-if="selectedWindow == 'result-display'">
            <result-display :lang="lang" :question="current_question" :players-prev="results_players_prev" :players-new="results_players_new" :self="nickname" key="result-display" />
          </template>
          
          <template v-else-if="selectedWindow == 'game-end-screen'">
            <game-end-screen :lang="lang" :players="results_players_new" :self="nickname" key="game-end-screen" />
          </template>
          
          <template v-else>
            <div class="compWindow" id="waiting-window" key="waiting">
              {{ lang["Waiting for players and server.."] }}
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

import PlayerList from './components/PlayerList.vue'
import NextButton from './components/NextButton.vue'

import LoginWindow from './components/LoginWindow.vue'
import LobbySelection from './components/LobbySelection.vue'
import LobbyMenu from './components/LobbyMenu.vue'
import QuestionCategoryBetting from './components/QuestionCategoryBetting.vue'
import QuestionVsAttacker from './components/QuestionVsAttacker.vue'
import QuestionAsker from './components/QuestionAsker.vue'
import QuestionEstimator from './components/QuestionEstimator.vue'
import ResultDisplay from './components/ResultDisplay.vue'
import GameEndScreen from './components/GameEndScreen.vue'

export default
{
  name: 'Gameshow',
  components: {
    LanguageSelector,
    CookieConsent,
    PlayerList,
    NextButton,
    LoginWindow,
    LobbySelection,
    LobbyMenu,
    QuestionCategoryBetting,
    QuestionVsAttacker,
    QuestionAsker,
    QuestionEstimator,
    ResultDisplay,
    GameEndScreen,
  },
  data: () => { return {
    lang: lang.en,
    question_sets: [],
    
    consent: false,
    selectedWindow: "loading",
    nickname: "",
    lobby: "",
    joined: false,
    admin: "",
    admin_plays: true, //only local, don't use on non-admin clients
    money: 1,
    jokers: 0,
    players: [],
    current_question: {id: 0, type: "", category: "", question: "", answers: [], correct_answer: 0, wrong_answers: []},
    
    results_players_prev: [],
    results_players_new: [],
    animation_in_progress: false,
    players_cached: [],
    
    event_stream: null,
    last_event_id: -1,
    event_queue: [],
    
    lobby_selection_params: {
      not_found: false,
      closed: false,
    },
    lobby_menu_params: {
      lobby_open: true,
      initial_money: "500",
      initial_jokers: "3",
      normal_q_money: "500",
      estimation_q_money: "1000",
      question_set: ""
    },
  }; },
  computed: {
    watch_only: function()
    {
      return ((this.admin == this.nickname && !this.admin_plays) || !this.joined);
    },
  },
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
      const name = await api.get_name();
      if (name != "") {
        this.nickname = name;
        const lobby_id = global.extract_lobby_id();
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
      const nickname = await api.set_name(name);
      if (nickname != "") {
        this.nickname = nickname;
        const lobby_id = global.extract_lobby_id();
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
      const lobby_id = result.lobby_id;
      this.lobby = lobby_id;
      this.admin = result.admin;
      result = await api.join_lobby(lobby_id);
      this.lobby_selection_params.not_found = result.not_found;
      this.lobby_selection_params.closed = result.closed;
      if (!result.valid) return;
      this.admin = result.admin;
      this.nickname = result.new_name;
      this.joined = true;
      this.selectedWindow = "lobby-menu";
      await this.setup_event_listener();
      window.history.pushState("lobby", "Gameshow Lobby", "#" + lobby_id);
    },
    join_lobby: async function(lobby_id)
    {
      if (!this.consent) return false;
      if (lobby_id == "") return false;
      const result = await api.join_lobby(lobby_id);
      this.lobby_selection_params.not_found = result.not_found;
      this.lobby_selection_params.closed = result.closed;
      if (!result.valid) return false;
      this.lobby = lobby_id;
      this.admin = result.admin;
      this.nickname = result.new_name;
      this.joined = true;
      this.selectedWindow = "lobby-menu";
      await this.setup_event_listener();
      window.history.pushState("lobby", "Gameshow Lobby", "#" + lobby_id);
      return true;
    },
    setup_event_listener: async function()
    {
      this.event_stream = await api.get_event_stream(this.lobby);
      this.event_stream.addEventListener("ping", () => { this.handle_event_queue(); });
      this.event_stream.addEventListener("game_event", (event) => {
        this.handle_new_event(JSON.parse(event.data));
      });
      this.event_queue = await api.get_events(this.lobby);
      this.handle_event_queue();
    },
    admin_left: async function()
    {
      this.admin_plays = false;
      this.joined = false;
    },
    start_game: async function(admin_plays)
    {
      //control that admin is in lobby or not
      this.admin_plays = admin_plays;
      if (this.admin == this.nickname)
      {
        if (!admin_plays && this.joined)
        {
          await api.leave_lobby(this.lobby);
          this.joined = false;
        }
        else if (admin_plays && !this.joined)
        {
          const result = await api.join_lobby(this.lobby);
          if (!result.valid) admin_plays = false;
          else
          {
            this.admin = result.admin;
            this.nickname = result.new_name;
            this.joined = true;
          }
        }
      }
      //start game
      const prevSelected = this.selectedWindow;
      this.waitForPlayers();
      if (!await api.next_state(this.lobby))
      {
        this.selectedWindow = prevSelected;
      }
    },
    bet_money: async function(money)
    {
      const prevSelected = this.selectedWindow;
      this.waitForPlayers();
      if (!await api.bet_money(this.lobby, money))
      {
        this.selectedWindow = prevSelected;
      }
    },
    attack_player: async function(player)
    {
      const prevSelected = this.selectedWindow;
      this.waitForPlayers();
      if (!await api.attack_player(this.lobby, player))
      {
        this.selectedWindow = prevSelected;
      }
    },
    select_answer: async function(answer)
    {
      const prevSelected = this.selectedWindow;
      this.waitForPlayers();
      if (!await api.answer_question(this.lobby, answer))
      {
        this.selectedWindow = prevSelected;
      }
    },
    get_joker: async function()
    {
      this.current_question.wrong_answers = await api.get_joker(this.lobby);
    },
    
    waitForPlayers: function()
    {
      this.selectedWindow = "";
    },
    finishedAnimation: function()
    {
      if (this.animation_in_progress)
      {
        this.animation_in_progress = false;
        this.players = this.players_cached;
      }
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
      this.finishedAnimation();
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
      this.finishedAnimation();
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
      this.finishedAnimation();
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
      this.finishedAnimation();
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
      this.finishedAnimation();
      this.current_question.type = "";
      
      this.results_players_new = event.player_data;
      this.selectedWindow = "game-end-screen";
    },
    eventBackToMenu: function(event)
    {
      this.finishedAnimation();
      this.current_question.type = "";
      
      this.lobby_menu_params.open = event.open;
      this.selectedWindow = "lobby-menu";
    },
    eventPlayerListUpdate: function(event)
    {
      this.players_cached = event.player_data;
      let found_myself = false;
      for (const player of this.players_cached)
      {
        if (player.name == this.nickname)
        {
          this.money = player.money;
          this.jokers = player.jokers;
          found_myself = true;
        }
      }
      this.joined = found_myself;
      if (!this.animation_in_progress) this.players = this.players_cached;
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

#waiting-window
{
  max-width: 60vw;
}

/*@media only screen and (max-width: 1000px)*/
@media (max-width: 50rem)
{
  .compWindow
  {
    font-size: large;
  }
  #waiting-window
  {
    max-width: 100vw;
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

input[type=button], input[type=submit], button
{
  cursor: pointer;
}
input[type=button][disabled], input[type=submit][disabled], button[disabled]
{
  cursor: not-allowed;
}
</style>
