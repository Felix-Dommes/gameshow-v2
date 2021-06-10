<template>
  <div class="compWindow">
    <!-- TODO
        field -> copy invite link
        lobby open checkbox -> change action
        admin plays or not checkbox => leave on start game
        allow joining while playing checkbox => send action on start game
        initial money -> change action
        initial jokers -> change action
        normal q money -> change action
        estimation q money -> change action
        question-selection -=> when and how?
        start button => action
        
        only editable if admin
        all directly sending checkboxes 2 sec disabled after click
        the normal inputs should only be synced to server when starting the lobby
    -->
    
    <div style="margin-bottom: 1ex;">
      <label for="invite-link">{{ lang['Invite link'] }}: </label>
      <input type="text" class="input" name="invite-link" :value="invite_link" readonly>
      <input type="button" class="button" :value="lang['Copy']" @click="copy_invite_link">
    </div>
    
    <div style="margin-bottom: 1ex;">
      <input type="checkbox" class="checkbox" name="lobby-open" v-model="lobby_open" :disabled="!admin">
      <label for="lobby-open"> {{ lang['Lobby open for new players'] }}</label>
      <br>
      <input type="checkbox" class="checkbox" name="admin-plays" v-model="admin_plays" :disabled="!admin">
      <label for="admin-plays"> {{ lang['Admin also plays'] }}</label>
      <br>
      <input type="checkbox" class="checkbox" name="open-while-playing" v-model="open_while_playing" :disabled="!admin">
      <label for="open-while-playing"> {{ lang['Lobby open while playing'] }}</label>
    </div>
    
    <table style="margin-bottom: 1ex;">
      <tr>
        <td><label for="initial-money">{{ lang['Initial money'] }}: </label></td>
        <td><input type="text" name="initial-money" v-model.number="initial_money" :disabled="!admin"></td>
      </tr>
      <tr>
        <td><label for="initial-jokers">{{ lang['Jokers'] }}: </label></td>
        <td><input type="text" name="initial-jokers" v-model.number="initial_jokers" :disabled="!admin"></td>
      </tr>
      <tr>
        <td><label for="normal-q-money">{{ lang['Normal question reward'] }}: </label></td>
        <td><input type="text" name="normal-q-money" v-model.number="normal_q_money" :disabled="!admin"></td>
      </tr>
      <tr>
        <td><label for="estimation-q-money">{{ lang['Estimation question reward'] }}: </label></td>
        <td><input type="text" name="estimation-q-money" v-model.number="estimation_q_money" :disabled="!admin"></td>
      </tr>
    </table>
    
    <div style="margin-bottom: 1em;">
      <label for="question-set">{{ lang['Question set'] }}: </label>
      <select name="question-set" v-model="question_set">
        <option value="" disabled>{{ lang['Select one'] }}</option>
        <option v-for="set in question_sets" :key="set" :value="set">{{ set }}</option>
        <option value="custom">{{ lang['Custom'] }}</option>
      </select>
      <template v-if="question_set == 'custom'">
        <br>
        <a href="questions-example.json">{{ lang['Download example'] }}</a>
        <br>
        <!-- TODO upload question file -->
      </template>
    </div>
    
    <input type="button" id="start" :value="lang['Start game']" :disabled="!admin">
  </div>
</template>

<script>
export default {
  name: "LobbyMenu",
  props: ["lang", "admin", "question_sets"],
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
    };
  },
  methods: {
    copy_invite_link: async function()
    {
      await navigator.clipboard.writeText(this.invite_link);
    }
  },
  mounted: function () {
    //document.getElementById("lobby-input").focus(); TODO
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

.error
{
  color: red;
  font-size: 60%;
}
</style>
