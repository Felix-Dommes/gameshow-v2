<template>
  <div>
    <div class="compWindow" style="padding: 1ex 1em">
      <span class="yellow">{{ lang["Players"] }}</span>
      <table class="nameList">
        <tr v-for="player of players" :key="player.name">
          <td>
            <span :class="{ admin: player.name == admin, yellow: player.name == self }">{{ player.name }}</span>
            ({{ player.jokers }}J)
            <span class="material-icons icon clickable" v-if="self == admin" @click="edit_player(player)">edit</span>
          </td>
          <td>-</td>
          <td>{{ player.money }} €</td>
          <template v-if="question_type == 'BettingQuestion'">
            <td><span class="material-icons icon">arrow_right_alt</span></td>
            <td>{{ player.money_bet != 0 ? player.money_bet + " €" : lang["None"] }}</td>
          </template>
        </tr>
      </table>
    </div>
    <transition name="background" appear>
      <div class="popup" v-show="edit_popup" @click.self="close_popup">
        <transition name="popup" appear>
          <div class="compWindow" v-if="edit_popup" @click.stop>
            <span class="yellow">{{ lang["Edit player"] }}</span>
            <span class="material-icons clickable" id="close-icon" @click="close_popup">close</span>
            <form @submit.prevent="save_player">
              <table>
                <tr>
                  <td><label>{{ editing_player.name }}</label></td>
                  <td><input type="button" :value="lang['Kick']" @click="kick_player"></td>
                </tr>
                <tr>
                  <td><label for="money">{{ lang["Money"] }}: </label></td>
                  <td><input type="number" id="money" v-model.number="editing_player.money" min="1"></td>
                </tr>
                <tr>
                  <td><label for="jokers">{{ lang["Jokers"] }}: </label></td>
                  <td><input type="number" id="jokers" v-model.number="editing_player.jokers" min="0"></td>
                </tr>
                <tr>
                  <td colspan="2"><input type="submit" id="save" :value="lang['Save']"></td>
                </tr>
              </table>
            </form>
          </div>
        </transition>
      </div>
    </transition>
  </div>
</template>

<script>
import api from '../assets/api.js'

export default {
  name: "PlayerList",
  props: ["lang", "players", "self", "admin", "lobby_id", "question_type"],
  data: function() {
    return {
      edit_popup: false,
      editing_player: {},
    };
  },
  methods: {
    edit_player: function(player)
    {
      this.editing_player = Object.assign({}, player);
      this.edit_popup = true;
    },
    close_popup: function()
    {
      this.edit_popup = false;
    },
    save_player: async function(event)
    {
      event.target.disabled = true;
      if (this.editing_player.money < 1) this.editing_player.money = 1;
      if (this.editing_player.jokers < 0) this.editing_player.jokers = 0;
      const res = await api.set_player_attributes(this.lobby_id, this.editing_player.name, this.editing_player.money, this.editing_player.jokers);
      event.target.disabled = false;
      if (res) this.close_popup();
    },
    kick_player: async function(event)
    {
      event.target.disabled = true;
      const res = await api.kick_player(this.lobby_id, this.editing_player.name);
      event.target.disabled = false;
      if (res) this.close_popup();
      if (res && this.admin == this.editing_player.name) this.$emit("admin-leaves");
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
table.nameList {
  border-spacing: 1ex 0;
  font-size: large;
  padding: 0;
  margin: 0;
}

table.nameList tr td:nth-child(2n + 3) {
  color: #ffdd77;
  text-align: right;
}

.icon {
  font-size: large;
  transform: translateY(3px);
}

.clickable {
  cursor: pointer;
}

.yellow {
  color: #ffdd77;
}

.admin {
  color: #b00000;
}


.popup {
  position: absolute;
  z-index: 900;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.popup-enter, .popup-leave-to
{
  transform: scale(0, 0);
}
.background-enter, .background-leave-to
{
  opacity: 0;
}
.popup-enter-active, .background-enter-active
{
  transition: all 0.25s ease-out;
}
.popup-leave-active, .background-leave-active
{
  transition: all 0.25s ease-in;
}

#close-icon
{
  float: right;
  font-size: xx-large;
}

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

input[type=button], input[type=submit]
{
  height: 3em;
  position: relative;
  top: -0.5ex;
}

input[type=button]#save, input[type=submit]#save
{
  width: 100%;
  height: 4em;
  position: static;
  top: 0;
}
</style>
