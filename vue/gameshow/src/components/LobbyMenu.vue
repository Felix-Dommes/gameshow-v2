<template>
  <div class="compWindow">
    <div style="margin-bottom: 1ex;">
      <label for="invite-link">{{ lang['Invite link'] }}: </label>
      <input type="text" id="invite-link" :value="invite_link" readonly autofocus>
      <input type="button" :value="lang['Copy']" @click="copy_invite_link">
    </div>
    
    <template v-if="admin">
      <div style="margin-bottom: 1ex;">
        <input type="checkbox" id="lobby-open" v-model="lobby_open" @change="update_lobby">
        <label for="lobby-open"> {{ lang['Lobby open for new players'] }}</label>
        <br>
        <input type="checkbox" id="admin-plays" v-model="admin_plays">
        <label for="admin-plays"> {{ lang['Admin also plays'] }}</label>
      </div>
      
      <table style="margin-bottom: 1ex;">
        <tr>
          <td><label for="initial-money">{{ lang['Initial money'] }}: </label></td>
          <td><input type="number" id="initial-money" min="1" v-model.number="initial_money" @change="update_lobby"></td>
        </tr>
        <tr>
          <td><label for="initial-jokers">{{ lang['Jokers'] }}: </label></td>
          <td><input type="number" id="initial-jokers" min="0" v-model.number="initial_jokers" @change="update_lobby"></td>
        </tr>
        <tr>
          <td><label for="normal-q-money">{{ lang['Normal question reward'] }}: </label></td>
          <td><input type="number" id="normal-q-money" min="1" v-model.number="normal_q_money" @change="update_lobby"></td>
        </tr>
        <tr>
          <td><label for="estimation-q-money">{{ lang['Estimation question reward'] }}: </label></td>
          <td><input type="number" id="estimation-q-money" min="1" v-model.number="estimation_q_money" @change="update_lobby"></td>
        </tr>
      </table>
      
      <div style="margin-bottom: 1em;">
        <label for="question-set">{{ lang['Question set'] }}: </label>
        <select id="question-set" v-model="question_set" @change="update_lobby">
          <option value="" disabled>{{ lang['Select one'] }}</option>
          <option v-for="set in question_sets" :key="set" :value="set">{{ set }}</option>
          <option value="custom">{{ lang['Custom'] }}</option>
        </select>
        <template v-if="question_set == 'custom'">
          <br>
          <a href="questions-example.json">{{ lang['Download example'] }}</a>
          <br>
          <label for="question-file-selector">{{ lang['Select file'] }}: </label>
          <input type="file" id="question-file-selector" accept="application/json,.json" @change="load_questions">
          <template v-if="error">
            <br>
            <span class="error">{{ error_msg }}</span>
          </template>
          <template v-else-if="success">
            <br>
            <span class="success">{{ success_msg }}</span>
          </template>
        </template>
      </div>
      
      <button type="button" id="start" :disabled="start_disabled" @click="start_game">
        <span v-if="out_of_sync" class="material-icons mirrored spinning">sync</span>
        <span v-else>{{ lang['Start game'] }}</span>
      </button>
    </template>
    
    <template v-else>
      <div style="margin-bottom: 1ex;">
        <input type="checkbox" id="lobby-open" v-model="sync_params.lobby_open" disabled>
        <label for="lobby-open"> {{ lang['Lobby open for new players'] }}</label>
      </div>
      
      <table style="margin-bottom: 1ex;">
        <tr>
          <td><label for="initial-money">{{ lang['Initial money'] }}: </label></td>
          <td><input type="number" id="initial-money" v-model.number="sync_params.initial_money" disabled></td>
        </tr>
        <tr>
          <td><label for="initial-jokers">{{ lang['Jokers'] }}: </label></td>
          <td><input type="number" id="initial-jokers" v-model.number="sync_params.initial_jokers" disabled></td>
        </tr>
        <tr>
          <td><label for="normal-q-money">{{ lang['Normal question reward'] }}: </label></td>
          <td><input type="number" id="normal-q-money" v-model.number="sync_params.normal_q_money" disabled></td>
        </tr>
        <tr>
          <td><label for="estimation-q-money">{{ lang['Estimation question reward'] }}: </label></td>
          <td><input type="number" id="estimation-q-money" v-model.number="sync_params.estimation_q_money" disabled></td>
        </tr>
      </table>
      
      <div style="margin-bottom: 1em;">
        <label for="question-set">{{ lang['Question set'] }}: </label>
        <select id="question-set" v-model="sync_params.question_set" disabled>
          <option value="" disabled>{{ lang['Select one'] }}</option>
          <option v-for="set in question_sets" :key="set" :value="set">{{ set }}</option>
          <option value="custom">{{ lang['Custom'] }}</option>
        </select>
      </div>
      
      <button type="button" id="start" disabled>
        <span>{{ lang['Start game'] }}</span>
      </button>
    </template>
  </div>
</template>

<script>
import api from '../assets/api.js'

export default {
  name: "LobbyMenu",
  props: ["lang", "admin", "lobby_id", "question_sets", "sync_params"],
  data: function () {
    return {
      invite_link: window.location.href,
      lobby_open: true,
      admin_plays: true,
      initial_money: 500,
      initial_jokers: 3,
      normal_q_money: 500,
      estimation_q_money: 1000,
      question_set: "",
      error: false,
      error_msg: "",
      success: false,
      success_msg: "",
      wait_for_server_start: false,
    };
  },
  computed: {
    out_of_sync: function() {
      return (Boolean(this.sync_params.lobby_open) != Boolean(this.lobby_open) || 
        Number(this.sync_params.initial_money) != Number(this.initial_money) ||
        Number(this.sync_params.initial_jokers) != Number(this.initial_jokers) ||
        Number(this.sync_params.normal_q_money) != Number(this.normal_q_money) ||
        Number(this.sync_params.estimation_q_money) != Number(this.estimation_q_money) ||
        this.sync_params.question_set != this.question_set);
    },
    start_disabled: function() {
      return (this.question_set == '' || this.out_of_sync || this.wait_for_server_start);
    },
  },
  methods: {
    copy_invite_link: async function()
    {
      await navigator.clipboard.writeText(this.invite_link);
    },
    update_lobby: async function()
    {
      await api.update_lobby(this.lobby_id, this.lobby_open, this.initial_money, this.initial_jokers, this.normal_q_money, this.estimation_q_money, this.question_set);
    },
    load_questions: async function(event)
    {
      const file_list = event.target.files;
      const file = file_list[0];
      if (file.size > 51200)
      {
        this.success = false;
        this.error_msg = this.lang["File is too large!"];
        this.error = true;
      }
      else
      {
        //read file and upload as soon as loaded
        const file_reader = new FileReader();
        file_reader.addEventListener("load", async (event) => {
          let questions;
          try
          {
            questions = JSON.parse(event.target.result);
          }
          catch (e)
          {
            this.success = false;
            this.error_msg = this.lang["Invalid JSON!"];
            this.error = true;
          }
          if (await api.upload_custom_questions(this.lobby_id, questions))
          {
            this.error = false;
            this.success_msg = this.lang["Questions uploaded!"];
            this.success = true;
          }
          else
          {
            this.success = false;
            this.error_msg = this.lang["Upload error!"];
            this.error = true;
          }
        });
        file_reader.readAsText(file);
      }
    },
    start_game: async function()
    {
      if (this.question_set == "" || (this.question_set == "custom" && !this.success))
      {
        alert(this.lang["Load questions before you start the game!"]);
        return;
      }
      if (this.out_of_sync)
      {
        alert(this.lang["Game settings out of sync, please wait!"]);
        return;
      }
      
      this.wait_for_server_start = true;
      this.$emit("start-game", this.admin_plays);
      setTimeout(function(comp) { comp.wait_for_server_start = false; }, 2000, this);
    },
  },
  mounted: function () {
    document.getElementById("invite-link").focus();
    //with a small delay, so that the server events get loaded first, ..
    setTimeout(() => {
      //.. set settings to those present from the server on menu reload
      this.lobby_open = Boolean(this.sync_params.lobby_open);
      this.initial_money = Number(this.sync_params.initial_money);
      this.initial_jokers = Number(this.sync_params.initial_jokers);
      this.normal_q_money = Number(this.sync_params.normal_q_money);
      this.estimation_q_money = Number(this.sync_params.estimation_q_money);
      this.question_set = this.sync_params.question_set;
    }, 500);
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
label
{
  font-size: 75%;
  margin-right: 1ex;
}

input[type=text], input[type=number]
{
  height: 3em;
  box-sizing: border-box;
  position: relative;
  top: -0.5ex;
}

input[type=button]
{
  height: 3em;
  position: relative;
  top: -0.5ex;
}

button#start
{
  width: 100%;
  height: 4em;
}

.mirrored
{
  transform: scaleX(-1);
}

@keyframes spin
{
  from { transform:rotate(0deg); }
  to { transform:rotate(360deg); }
}
.spinning
{
  animation-name: spin;
  animation-duration: 1s;
  animation-timing-function: linear;
  animation-iteration-count: infinite;
}

input[type=checkbox]
{
  transform: scale(2);
  margin: 1ex;
  position: relative;
  top: -0.5ex;
}

select
{
  transform: scale(1.5);
  margin: 1ex 3em;
  position: relative;
  top: -0.5ex;
}

a
{
  font-size: 60%;
}

input[type=file]
{
  height: 3em;
  box-sizing: border-box;
  position: relative;
  top: -0.5ex;
}

.success
{
  color: green;
  font-size: 60%;
}

.error
{
  color: red;
  font-size: 60%;
}
</style>
