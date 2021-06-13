<template>
  <div class="compWindow">
    <div style="margin-bottom: 1ex;">
      <label for="invite-link">{{ lang['Invite link'] }}: </label>
      <input type="text" class="input" name="invite-link" id="invite-link" :value="invite_link" readonly autofocus>
      <input type="button" class="button" :value="lang['Copy']" @click="copy_invite_link">
    </div>
    
    <template v-if="admin">
      <div style="margin-bottom: 1ex;">
        <input type="checkbox" class="checkbox" name="lobby-open" v-model="lobby_open" @change="update_lobby">
        <label for="lobby-open"> {{ lang['Lobby open for new players'] }}</label>
        <br>
        <input type="checkbox" class="checkbox" name="admin-plays" v-model="admin_plays">
        <label for="admin-plays"> {{ lang['Admin also plays'] }}</label>
        <br>
        <input type="checkbox" class="checkbox" name="open-while-playing" v-model="open_while_playing">
        <label for="open-while-playing"> {{ lang['Lobby open while playing'] }}</label>
      </div>
      
      <table style="margin-bottom: 1ex;">
        <tr>
          <td><label for="initial-money">{{ lang['Initial money'] }}: </label></td>
          <td><input type="text" name="initial-money" v-model.number="initial_money" @change="update_lobby"></td>
        </tr>
        <tr>
          <td><label for="initial-jokers">{{ lang['Jokers'] }}: </label></td>
          <td><input type="text" name="initial-jokers" v-model.number="initial_jokers" @change="update_lobby"></td>
        </tr>
        <tr>
          <td><label for="normal-q-money">{{ lang['Normal question reward'] }}: </label></td>
          <td><input type="text" name="normal-q-money" v-model.number="normal_q_money" @change="update_lobby"></td>
        </tr>
        <tr>
          <td><label for="estimation-q-money">{{ lang['Estimation question reward'] }}: </label></td>
          <td><input type="text" name="estimation-q-money" v-model.number="estimation_q_money" @change="update_lobby"></td>
        </tr>
      </table>
      
      <div style="margin-bottom: 1em;">
        <label for="question-set">{{ lang['Question set'] }}: </label>
        <select name="question-set" v-model="question_set" @change="update_lobby">
          <option value="" disabled>{{ lang['Select one'] }}</option>
          <option v-for="set in question_sets" :key="set" :value="set">{{ set }}</option>
          <option value="custom">{{ lang['Custom'] }}</option>
        </select>
        <template v-if="question_set == 'custom'">
          <br>
          <a href="questions-example.json">{{ lang['Download example'] }}</a>
          <br>
          <label for="question-file-selector">{{ lang['Select file'] }}: </label>
          <input type="file" name="question-file-selector" id="question-file-selector" accept="application/json,.json" @change="load_questions">
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
      
      <input type="button" id="start" :value="lang['Start game']" :disabled="this.question_set == ''" @click="start_game">
    </template>
    
    <template v-else>
      <div style="margin-bottom: 1ex;">
        <input type="checkbox" class="checkbox" name="lobby-open" v-model="sync_params.lobby_open" disabled>
        <label for="lobby-open"> {{ lang['Lobby open for new players'] }}</label>
      </div>
      
      <table style="margin-bottom: 1ex;">
        <tr>
          <td><label for="initial-money">{{ lang['Initial money'] }}: </label></td>
          <td><input type="text" name="initial-money" v-model.number="sync_params.initial_money" disabled></td>
        </tr>
        <tr>
          <td><label for="initial-jokers">{{ lang['Jokers'] }}: </label></td>
          <td><input type="text" name="initial-jokers" v-model.number="sync_params.initial_jokers" disabled></td>
        </tr>
        <tr>
          <td><label for="normal-q-money">{{ lang['Normal question reward'] }}: </label></td>
          <td><input type="text" name="normal-q-money" v-model.number="sync_params.normal_q_money" disabled></td>
        </tr>
        <tr>
          <td><label for="estimation-q-money">{{ lang['Estimation question reward'] }}: </label></td>
          <td><input type="text" name="estimation-q-money" v-model.number="sync_params.estimation_q_money" disabled></td>
        </tr>
      </table>
      
      <div style="margin-bottom: 1em;">
        <label for="question-set">{{ lang['Question set'] }}: </label>
        <select name="question-set" v-model="sync_params.question_set" disabled>
          <option value="" disabled>{{ lang['Select one'] }}</option>
          <option v-for="set in question_sets" :key="set" :value="set">{{ set }}</option>
          <option value="custom">{{ lang['Custom'] }}</option>
        </select>
      </div>
      
      <input type="button" id="start" :value="lang['Start game']" disabled>
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
      open_while_playing: true,
      initial_money: "500",
      initial_jokers: "3",
      normal_q_money: "500",
      estimation_q_money: "1000",
      question_set: "",
      error: false,
      error_msg: "",
      success: false,
      success_msg: "",
    };
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
          try
          {
            const questions = JSON.parse(event.target.result);
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
          }
          catch (e)
          {
            this.success = false;
            this.error_msg = this.lang["Invalid JSON!"];
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
      if (Boolean(this.sync_params.lobby_open) != Boolean(this.lobby_open) || 
        Number(this.sync_params.initial_money) != Number(this.initial_money) ||
        Number(this.sync_params.initial_jokers) != Number(this.initial_jokers) ||
        Number(this.sync_params.normal_q_money) != Number(this.normal_q_money) ||
        Number(this.sync_params.estimation_q_money) != Number(this.estimation_q_money) ||
        this.sync_params.question_set != this.question_set )
      {
        alert(this.lang["Game settings out of sync, please wait!"]);
        return;
      }
      
      if (this.lobby_open != this.open_while_playing)
      {
        this.lobby_open = this.open_while_playing;
        await this.update_lobby();
      }
      this.$emit("start-game", this.admin_plays);
    },
  },
  mounted: function () {
    document.getElementById("invite-link").focus();
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

input[type=text]
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

input#start
{
  width: 100%;
  height: 4em;
  position: static;
  top: 0;
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
