<template>
  <div class="compWindow">
    <form @submit.prevent="submit_name">
      <label for="name">{{ lang["Name"] }}:</label>
      <input type="text" name="name" :placeholder="'<'+lang['Name']+'>'" v-model.trim="nickname" autofocus id="login-input" class="input" maxlength="25">
      <br>
      <template v-if="error">
        <span class="error">{{ error_msg }}</span>
        <br>
      </template>
      <input type="submit" :value="lang['Submit']" id="login-submit" class="button">
    </form>
  </div>
</template>

<script>
export default {
  name: 'LoginWindow',
  props: ["lang"],
  data: function() { return {
    nickname: "",
    error: false,
    error_msg: "",
  }; },
  methods: {
    submit_name: function()
    {
      if (this.nickname == "")
      {
        this.error_msg = this.lang["Name must not be empty!"];
        this.error = true;
      }
      else if (this.nickname.length > 25)
      {
        this.error_msg = this.lang["Name is too long! At most 25 characters!"];
        this.error = true;
      }
      else
      {
        this.error = false;
        let button = document.getElementById("login-submit");
        button.setAttribute("disabled", "disabled");
        this.$emit("set-name", this.nickname);
        setTimeout(() => { button.removeAttribute("disabled"); }, 2000);
      }
    },
  },
  mounted: function()
  {
    document.getElementById("login-input").focus();
  },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
label
{
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
