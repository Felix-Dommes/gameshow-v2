<template>
  <div class="compWindow">
    <span class="yellow">{{ lang["Bet money for the question"] }}</span>
    <p class="text-60">
      {{ lang["If you answer wrongly, you pay the bet money, otherwise you get the bet money!"] }}
    </p>
    <span class="yellow text-80">{{ lang["Question category"] }}: </span>
    <span class="text-70">{{ question.category }}</span>
    <br>
    <form @submit.prevent="bet">
      <input type="number" id="betting-input" :placeholder="lang['<Amount>']" v-model.number="bet_value" min="1" :max="max_bet" autofocus  :disabled="watch_only">
      <input type="submit" :value="lang['Submit']" :disabled="watch_only || bet_value == ''">
    </form>
  </div>
</template>

<script>
export default {
  name: "QuestionCategoryBetting",
  props: ["lang", "watch_only", "question", "max_bet"],
  data: function () {
    return {
      bet_value: "",
    };
  },
  methods: {
    bet: function()
    {
      if (this.watch_only) return;
      if (this.bet_value == "")
      {
        alert(this.lang["You must bet money!"]);
      }
      else if (this.bet_value < 1 || this.bet_value > this.max_bet)
      {
        alert(this.lang["Invalid bet! Must be > 1 and <= your money!"]);
      }
      else
      {
        this.$emit('bet-money', this.bet_value);
      }
    },
  },
  mounted: function() {
    document.getElementById("betting-input").focus();
  },
};
</script>


<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
input[type=number]
{
  width: 100%;
  height: 3em;
  box-sizing: border-box;
  position: relative;
  top: 0.25ex;
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
