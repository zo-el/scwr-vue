<template>
  <progress v-if="loading"></progress>
  <div v-else>
    <div class="alert" v-if="error">
      Error fetching the executed instances: {{ error.message }}.
    </div>
    <div v-else-if="hashes && hashes.length > 0">
      <ExecutedInstanceDetail
        v-for="(hash, i) in hashes"
        :key="i"
        :executed-instance-hash="hash"
        @executed-instance-deleted="fetchExecutedInstance()"
      >
      </ExecutedInstanceDetail>
    </div>
    <div class="alert" v-else>No executed instances found.</div>
  </div>
</template>

<script lang="ts">
import {
  ActionHash,
  AgentPubKey,
  AppClient,
  EntryHash,
  HolochainError,
  Link,
  NewEntryAction,
  Record,
  SignalType,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { ComputedRef, inject, toRaw } from "vue";
import ExecutedInstanceDetail from "./ExecutedInstanceDetail.vue";
import { ScwrSignal } from "./types";

export default {
  components: {
    ExecutedInstanceDetail,
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: false,
      error: undefined,
    };
  },
  async mounted() {
    await this.fetchExecutedInstance();
    toRaw(this.client).on("signal", signal => {
      if (signal.zome_name !== "scwr") return;
      const payload = signal.payload as ScwrSignal;
      if (payload.type !== "EntryCreated") return;
      if (payload.app_entry.type !== "ExecutedInstance") return;
      if (this.hashes) this.hashes.push(payload.action.hashed.hash);
    });
  },
  methods: {
    async fetchExecutedInstance() {
      try {
        this.loading = true;
        const links: Array<Link> = await this.client.callZome({
          cap_secret: null,
          role_name: "scwr",
          zome_name: "scwr",
          fn_name: "get_all_instances",
          payload: null,
        });
        this.hashes = links.map(l => l.target);
      } catch (e) {
        this.error = e as HolochainError;
      } finally {
        this.loading = false;
      }
    },
  },
  setup() {
    const client = (inject("client") as ComputedRef<AppClient>).value;
    return { client };
  },
};
</script>
