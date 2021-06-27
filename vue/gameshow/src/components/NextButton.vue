<template>
  <button :disabled="button_disabled" @click="go_next">
    <span v-if="selected_window == 'result-display'">
      {{ lang["Next question"] }}
    </span>
    <span v-else-if="selected_window == 'game-end-screen'">
      {{ lang["Back to menu"] }}
    </span>
    <span v-else>
      {{ lang["Force to go on"] }}
    </span>
  </button>
</template>

<script>
import api from '../assets/api.js'

export default {
  name: "NextButton",
  props: ["lang", "lobby_id", "selected_window"],
  data: function () {
    return {
      waiting_for_server: false,
    };
  },
  computed: {
    button_disabled: function()
    {
      return (this.waiting_for_server || this.selected_window == 'lobby-menu');
    }
  },
  methods: {
    go_next: async function()
    {
      this.waiting_for_server = true;
      if (await api.next_state(this.lobby_id))
      {
        setTimeout(function(comp) { comp.waiting_for_server = false; }, 2000, this);
      }
      else
      {
        this.waiting_for_server = false;
      }
    }
  },
};
</script>

<style scoped>
button
{
  width: 100%;
  height: 4em;
}
</style>
