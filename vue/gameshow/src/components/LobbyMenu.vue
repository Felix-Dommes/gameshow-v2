<template>
  <div class="compWindow">
    <!-- TODO
        admin plays or not checkbox => leave on start game
        lobby open checkbox
        initial money
        initial jokers
        normal q money
        estimation q money
        question-selection
        allow joining while playing checkbox
        start button
        
        field -> copy invite link
        
        only editable if admin
        all directly sending checkboxes 2 sec disabled after click
        the normal inputs should only be synced to server when starting the lobby
    -->
    <form @submit.prevent="button_create_lobby">
      <input type="submit" :value="lang['Create lobby']" id="lobby-create" class="button">
    </form>
    <br>
    <form @submit.prevent="button_join_lobby">
      <label for="lobbyID">{{ lang["Join lobby"] }}:</label>
      <input type="text" name="lobbyID" :placeholder="'<'+lang['Lobby ID']+'>'" v-model.trim="lobby_id" autofocus id="lobby-input" class="input">
      <br>
      <template v-if="error">
        <span class="error">{{ error_msg }}</span>
        <br>
      </template>
      <template v-if="notFound">
        <span class="error">{{ lang["Lobby ID was not found!"] }}</span>
        <br>
      </template>
      <input type="submit" :value="lang['Join']" id="lobby-join" class="button">
    </form>
  </div>
</template>

<script>
export default {
  name: "LobbyMenu",
  props: ["lang", "admin"],
  data: function () {
    return {
      //
    };
  },
  methods: {
    //
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

input.input
{
  height: 3em;
  box-sizing: border-box;
  position: relative;
  top: -0.5ex;
}

input.button
{
  width: 100%;
  height: 4em;
}

.error
{
  color: red;
  font-size: 60%;
}
</style>
