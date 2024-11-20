<template>
  <div>
    <div>
      <a href="https://developer.holochain.org/get-started/" target="_blank">
        <img
          src="./assets/holochainLogo.svg"
          class="logo holochain"
          alt="holochain logo"
        />
      </a>
    </div>
    <h1>Holochain Vue hApp</h1>
    <div>
      <div class="card">
        <p v-if="loading">connecting...</p>
        <p v-else-if="error"></p>
        <p v-else>Client is connected.</p>
      </div>
      <p>
        Import scaffolded components into <code>src/App.vue</code> to use your
        hApp
      </p>
      <div class="main-content">
        <!-- <AllInstances/> -->
        <!-- <CreateExecutedInstance/> -->
        <ExecutedInstanceDetail/>
      </div>
      <p class="read-the-docs">Click on the Holochain logo to learn more</p>
    </div>
  </div>
</template>

<script lang="ts">
import { AppClient, AppWebsocket, HolochainError } from "@holochain/client";
import { computed } from "vue";
import AllInstances from "./scwr/scwr/AllInstances.vue"
import CreateExecutedInstance from "./scwr/scwr/CreateExecutedInstance.vue"
import ExecutedInstanceDetail from "./scwr/scwr/ExecutedInstanceDetail.vue"


export default {
  components: {},
  data(): {
    client: AppClient | undefined;
    error: HolochainError | undefined;
    loading: boolean;
  } {
    return {
      client: undefined,
      error: undefined,
      loading: false,
    };
  },
  async mounted() {
    try {
      this.loading = true;
      this.client = await AppWebsocket.connect();
    } catch (e) {
      this.error = e as HolochainError;
    } finally {
      this.loading = false;
    }
  },
  provide() {
    return {
      client: computed(() => this.client),
    };
  },
};
</script>

<style scoped>
.logo {
  height: 15em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
  width: auto;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.holochain:hover {
  filter: drop-shadow(0 0 2em #61dafbaa);
}

.card {
  padding: 2em;
}

.read-the-docs {
  color: #888;
}
</style>
