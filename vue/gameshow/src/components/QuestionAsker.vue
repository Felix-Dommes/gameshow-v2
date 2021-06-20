<template>
  <div class="compWindow">
    <div class="joker" v-if="question.type == 'BettingQuestion' || question.type == 'NormalQuestion'" :disabled="!jokerAvailable || usedJoker || watch_only" @click="jokerClick">
      50:50
    </div>
    <table class="question-design">
      <tr>
        <td colspan="2" class="quizElement">{{ question.question }}</td>
      </tr>
      <tr></tr>
      <tr>
        <td class="quizElement answer" v-bind:class="[quizAnsClasses[0], computedQuizAnsClasses[0]]" @mouseover="qE_mouseOver(0)" @mouseout="qE_mouseOut(0)" @click="qE_click(0)">a) {{ question.answers[0] }}</td>
        <td class="quizElement answer" v-bind:class="[quizAnsClasses[1], computedQuizAnsClasses[1]]" @mouseover="qE_mouseOver(1)" @mouseout="qE_mouseOut(1)" @click="qE_click(1)">b) {{ question.answers[1] }}</td>
      </tr>
      <tr>
        <td class="quizElement answer" v-bind:class="[quizAnsClasses[2], computedQuizAnsClasses[2]]" @mouseover="qE_mouseOver(2)" @mouseout="qE_mouseOut(2)" @click="qE_click(2)">c) {{ question.answers[2] }}</td>
        <td class="quizElement answer" v-bind:class="[quizAnsClasses[3], computedQuizAnsClasses[3]]" @mouseover="qE_mouseOver(3)" @mouseout="qE_mouseOut(3)" @click="qE_click(3)">d) {{ question.answers[3] }}</td>
      </tr>
    </table>
  </div>
</template>

<script>
export default {
  name: "QuestionAsker",
  props: ["lang", "watch_only", "question", "jokerAvailable"],
  data: function () {
    return {
      quizAnsClasses: [
        { "mouseover": false, "clicked": false },
        { "mouseover": false, "clicked": false },
        { "mouseover": false, "clicked": false },
        { "mouseover": false, "clicked": false }
      ],
      answerSelected: false,
      usedJoker: false,
    };
  },
  computed: {
    computedQuizAnsClasses: function()
    {
      let cQAC = [{ "wrong": false }, { "wrong": false }, { "wrong": false }, { "wrong": false }];
      for (const wrong_answer of this.question.wrong_answers)
      {
        cQAC[wrong_answer - 1]["wrong"] = true;
      }
      return cQAC;
    }
  },
  methods: {
    qE_mouseOver: function(ansID)
    {
      if (!this.answerSelected && !this.watch_only) this.quizAnsClasses[ansID]["mouseover"] = true;
    },
    qE_mouseOut: function(ansID)
    {
      if (!this.answerSelected && !this.watch_only) this.quizAnsClasses[ansID]["mouseover"] = false;
    },
    qE_click: function(ansID)
    {
      if (this.answerSelected || this.watch_only) return;
      if (this.question.wrong_answers.includes(ansID + 1)) return; //do not allow click on wrong answer
      for (var i=0; i<4; i++)
      {
        this.quizAnsClasses[i]["mouseover"] = false;
        this.quizAnsClasses[i]["clicked"] = false;
      }
      this.quizAnsClasses[ansID]["clicked"] = true;
      this.answerSelected = true;
      this.$emit("answered", ansID + 1);
    },
    jokerClick: function()
    {
      if (!this.jokerAvailable || this.usedJoker || this.watch_only) return;
      this.usedJoker = true;
      this.$emit("joker");
    }
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
.mouseover
{
  background: linear-gradient(0deg, #000080 0%, #2233FF 50%, #000080 100%);
}
.clicked
{
  background: linear-gradient(0deg, #BB7700 0%, #FFCC33 50%, #BB7700 100%);
}
.wrong
{
  background: linear-gradient(0deg, #AA0000 0%, #FF5533 50%, #AA0000 100%);
}

.joker
{
  display: inline-block;
  border: 1px groove #DDDDDD;
  border-radius: 6em / 3em;
  box-shadow: 0 0 1px 0px white inset, 0 0 1px 0px white;
  background: linear-gradient(-15deg, #000055 0%, #1133FF 50%, #000055 100%);
  padding: 0.5ex 0.5em;
  text-align: center;
}
.joker:hover
{
  background: linear-gradient(-15deg, #883300 0%, #FFAA00 50%, #883300 100%);
}
.joker:active
{
  background: linear-gradient(-15deg, #FFAA00 0%, #AA5500 50%, #FFAA00 100%);
}
.joker[disabled]
{
  border-color: #999999;
  background: linear-gradient(-15deg, #111111 0%, #113355 50%, #111111 100%);
  color: #BBBBBB;
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
