<template>
  <div>
    <div class="compWindow" style="padding: 1ex 1em;">
      <span class="yellow">{{ lang["The Players' Answers"] }}</span>
      <table class="nameList">
        <tr v-for="(player, index) of playersPrev" :key="player.name">
          <td><span :class="{ yellow: player.name == self }">{{ player.name }}</span></td>
          <template v-if="question.type == 'BettingQuestion'">
            <td>{{ lang["bets"] }}</td>
            <td>{{ showBetsOrVersus ? (player.money_bet != 0 ? player.money_bet + " €" : lang["None"]) : "???" }}</td>
            <td>{{ lang["and assumes it is"] }}</td>
          </template>
          <template v-else-if="question.type == 'VersusQuestion'">
            <td>{{ lang["attacks;results-vs"] }}</td>
            <td>{{ showBetsOrVersus ? (player.vs_player != "" ? player.vs_player : lang["Nobody"]) : "???" }}</td>
            <td>{{ lang["and assumes it is;results-vs"] }}</td>
          </template>
          <template v-else-if="question.type == 'NormalQuestion'">
            <td>{{ lang["assumes it is"] }}</td>
          </template>
          <template v-else>
            <td>{{ lang["says"] }}</td>
          </template>
          <template v-if="question.type == 'EstimationQuestion'">
            <td>{{ showAnswers ? (player.answer != 0 ? player.answer : lang["Nothing"]) : "???" }}</td>
          </template>
          <template v-else>
            <td>{{ showAnswers ? answerChar[player.answer] : "?)" }}</td>
          </template>
          <template v-if="showCorrectAnswer">
            <td><span class="material-icons icon">arrow_right_alt</span></td>
            <td>{{ (playersNew[index].money > player.money ? "+" : "") + (playersNew[index].money - player.money).toString() }} €</td>
          </template>
        </tr>
      </table>
    </div>
    <div class="compWindow">
      <table class="question-design">
        <tr><td colspan="2" class="quizElement">{{ question.question }}</td></tr>
        <tr></tr>
        <template v-if="question.type != 'EstimationQuestion'">
          <tr>
            <td class="quizElement answer" v-bind:class="quizAnsClasses[0]">a) {{ question.answers[0] }}</td>
            <td class="quizElement answer" v-bind:class="quizAnsClasses[1]">b) {{ question.answers[1] }}</td>
          </tr>
          <tr>
            <td class="quizElement answer" v-bind:class="quizAnsClasses[2]">c) {{ question.answers[2] }}</td>
            <td class="quizElement answer" v-bind:class="quizAnsClasses[3]">d) {{ question.answers[3] }}</td>
          </tr>
        </template>
        <template v-else>
          <tr>
            <td colspan="2" style="font-size: large;">{{ lang["Correct answer"] }}: {{ showCorrectAnswer ? question.correct_answer : "???" }}</td>
          </tr>
        </template>
      </table>
    </div>
  </div>
</template>

<script>
export default {
  name: "ResultDisplay",
  props: ["lang", "question", "playersPrev", "playersNew", "self"],
  data: function () {
    return {
      quizAnsClasses: [
        { "correct": false, "clicked": false },
        { "correct": false, "clicked": false },
        { "correct": false, "clicked": false },
        { "correct": false, "clicked": false }
      ],
      showBetsOrVersus: false,
      showAnswers: false,
      showCorrectAnswer: false,
      timeouts: [],
    };
  },
  computed: {
    answerChar: function()
    {
      return [this.lang["Nothing"], "a)", "b)", "c)", "d)"];
    },
  },
  methods: {
    revealBets: function()
    {
      this.showBetsOrVersus = true;
    },
    revealAnswers: function()
    {
      this.showAnswers = true;
      if (this.question.type != "EstimationQuestion")
      {
        for (const player of this.playersPrev)
        {
          if (player.name == this.self && player.answer != 0)
          {
            this.quizAnsClasses[player.answer - 1]["clicked"] = true;
          }
        }
      }
    },
    revealCorrectAnswer: function()
    {
      this.showCorrectAnswer = true;
      if (this.question.type != "EstimationQuestion")
      {
        this.quizAnsClasses[this.question.correct_answer - 1]["correct"] = true;
      }
    },
    
    removeTimeouts: function()
    {
      for (const timeout of this.timeouts)
      {
        clearTimeout(timeout);
      }
      this.timeouts = [];
    },
  },
  mounted: function() {
    this.timeouts.push( setTimeout(function(comp) { comp.revealBets(); }, 3000, this) );
    this.timeouts.push( setTimeout(function(comp) { comp.revealAnswers(); }, 8000, this) );
    this.timeouts.push( setTimeout(function(comp) { comp.revealCorrectAnswer(); }, 15000, this) );
  },
  beforeUnmount: function() { //TODO why is this not called?
    this.removeTimeouts();
  },
};
</script>


<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
table.question-design
{
  border-spacing: 1em 1ex;
  max-width: 60vw;
}

/*@media only screen and (max-width: 1000px)*/
@media (max-width: 50rem)
{
  table.question-design
  {
    max-width: 100vw;
  }
}

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

.icon {
  font-size: large;
  transform: translateY(3px);
}

.quizElement
{
  background: linear-gradient(0deg, #0000C0 0%, #000080 50%, #0000C0 100%);
  text-align: center;
  padding: 1ex 1em;
  border-radius: 1ex;
}
.answer
{
  text-align: left;
  font-size: 75%;
  width: 50%;
}
.clicked
{
  background: linear-gradient(0deg, #BB7700 0%, #FFCC33 50%, #BB7700 100%);
}
.correct
{
  background: linear-gradient(0deg, #007700 0%, #33DD33 50%, #007700 100%);
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
