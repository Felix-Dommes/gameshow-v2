<template>
  <div class="compWindow">
    <span class="yellow">Ergebnisse</span>
    <table class="nameList">
      <tr v-for="(player, index) of sortedPlayers" :key="player.name">
        <td>{{ index + 1 }}. <span :class="{ yellow: player.name == self }">{{ player.name }}</span></td>
        <td>-</td>
        <td>{{ player.money }} €</td>
      </tr>
    </table>
  </div>
</template>

<script>
export default {
  name: "GameEndScreen",
  props: ["lang", "players", "self"],
  computed: {
    sortedPlayers: function()
    {
      let copy = Array.from(this.players);
      copy.sort(function(left, right) {
        return right.money - left.money; //sort descending by money
      });
      return copy;
    },
  },
};
</script>


<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
table.nameList
{
  border-spacing: 1ex 0;
  font-size: large;
  padding: 0;
  margin: 0;
}
table.nameList tr td:nth-child(2n+3)
{
  color: #FFDD77;
  text-align: right;
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
