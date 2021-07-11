<template>
  <div v-if="selected_window != 'lobby-menu'">
    <input type="checkbox" id="lobby-open-second" v-model="lobby_open" @change="update_lobby" :disabled="!admin">
    <label for="lobby-open-second"> {{ lang['Lobby open'] }}</label>
  </div>
</template>

<script>
import api from '../assets/api.js'

export default {
  name: "LobbyOpenCheckbox",
  props: ["lang", "admin", "lobby_id", "sync_params", "selected_window"],
  data: function () {
    return {
      lobby_open: true,
    };
  },
  methods: {
    update_lobby: async function()
    {
      await api.update_lobby(this.lobby_id, this.lobby_open, this.sync_params.initial_money, this.sync_params.initial_jokers, this.sync_params.normal_q_money, this.sync_params.estimation_q_money, this.sync_params.question_set);
    },
  },
  watch: {
    /*eslint no-unused-vars: ["error", { "argsIgnorePattern": "^_" }]*/
    sync_params: function(new_val, _old_val)
    {
      this.lobby_open = new_val.lobby_open;
    },
  },
  mounted: function () {
    //with a small delay, so that the server events get loaded first, ..
    setTimeout(() => {
      //.. set settings to those present from the server on menu reload
      this.lobby_open = Boolean(this.sync_params.lobby_open);
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

input[type=checkbox]
{
  transform: scale(1.5);
  margin: 1ex;
  position: relative;
  top: -0.5ex;
}
</style>
