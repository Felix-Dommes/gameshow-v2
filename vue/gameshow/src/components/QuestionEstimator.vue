<template>
  <div class="compWindow">
    <div class="quizElement" style="margin-bottom: 1ex;">{{ question.question }}</div>
    <span>{{ lang["Enter your estimation"] }}: </span>
    <form @submit.prevent="estimate">
      <input type="number" id="estimator-input" :placeholder="lang['<Estimation>']" v-model.number="estimation" min="1" autofocus><br>
      <input type="submit" :value="lang['Submit']" :disabled="watch_only || estimation == ''">
    </form>
  </div>
</template>

<script>
export default {
  name: "QuestionEstimator",
  props: ["lang", "watch_only", "question"],
  data: function () {
    return {
      estimation: "",
    };
  },
  methods: {
    estimate: function()
    {
      if (this.estimation == "")
      {
        alert(this.lang["Enter your estimation first!"]);
      }
      else if (this.estimation < 1)
      {
        alert(this.lang["Estimation must be at least 1!"]);
      }
      else
      {
        this.$emit('answered', this.estimation);
      }
    }
  },
  mounted: function() {
    document.getElementById("estimator-input").focus();
  },
};
</script>


<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.quizElement
{
  background: linear-gradient(0deg, #0000C0 0%, #000080 50%, #0000C0 100%);
  text-align: center;
  padding: 1ex 1em;
  border-radius: 1ex;
}

input[type=number]
{
  width: 100%;
  height: 3em;
  box-sizing: border-box;
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
