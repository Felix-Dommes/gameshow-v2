<template>
  <div class="compWindow">
    <span class="yellow">{{ lang["Attack a fellow player"] }}</span>
    <p class="text-60">
      {{ lang["If you answer correctly, your enemy's money is halved! But else it is doubled!"] }}
    </p>
    <span class="yellow text-80">{{ lang["Question category"] }}: </span>
    <span class="text-70">{{ question.category }}</span>
    <br>
    <form @submit.prevent="attack">
      <select id="attacking-select" v-model="selectedPlayer" autofocus :disabled="watch_only">
        <option value="" disabled>{{ lang["Select a player"] }}</option>
        <template v-for="player of players">
          <option v-if="player.name != self" :key="player.name" :value="player.name">{{ player.name }}</option>
        </template>
      </select>
      <input type="submit" :value="lang['Submit']" :disabled="watch_only || selectedPlayer == ''">
    </form>
  </div>
</template>

<script>
export default {
  name: "QuestionVsAttacker",
  props: ["lang", "watch_only", "question", "players", "self"],
  data: function () {
    return {
      selectedPlayer: "",
    };
  },
  methods: {
    attack: function()
    {
      if (this.selectedPlayer == "")
      {
        alert(this.lang["You must select a player!"]);
      }
      else
      {
        this.$emit('attack-player', this.selectedPlayer);
      }
    },
  },
  mounted: function() {
    document.getElementById("attacking-select").focus();
  },
};
</script>


<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
select
{
  width: 100%;
  height: 3em;
}

input[type=submit]
{
  width: 100%;
  height: 4em;
}

.yellow {
  color: #ffdd77;
}

.text-80 {
  font-size: 90%;
}
.text-70 {
  font-size: 70%;
}
.text-60 {
  font-size: 60%;
}
</style>
