<template>
  <div class="compWindow">
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
      <input type="submit" :value="lang['Join']" id="lobby-join" class="button">
    </form>
  </div>
</template>

<script>
export default {
  name: "LobbySelection",
  props: ["lang"],
  data: function () {
    return {
      lobby_id: "",
      error: false,
      error_msg: "",
    };
  },
  methods: {
    button_create_lobby: function () {
      let button1 = document.getElementById("lobby-create");
      let button2 = document.getElementById("lobby-join");
      button1.setAttribute("disabled", "disabled");
      button2.setAttribute("disabled", "disabled");
      this.$emit("create-lobby");
      setTimeout(() => {
        button1.removeAttribute("disabled");
        button2.removeAttribute("disabled");
      }, 2000);
    },
    button_join_lobby: function () {
      if (this.lobby_id == "") {
        this.error = true;
        this.error_msg = this.lang["Lobby ID must not be empty!"];
      }
      else {
        this.error = false;
        let button1 = document.getElementById("lobby-create");
        let button2 = document.getElementById("lobby-join");
        button1.setAttribute("disabled", "disabled");
        button2.setAttribute("disabled", "disabled");
        this.$emit("join-lobby", this.lobby_id);
        setTimeout(() => {
          button1.removeAttribute("disabled");
          button2.removeAttribute("disabled");
        }, 2000);
      }
    },
  },
  mounted: function () {
    document.getElementById("lobby-input").focus();
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
