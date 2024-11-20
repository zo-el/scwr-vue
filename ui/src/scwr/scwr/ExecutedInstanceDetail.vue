<template>
  <div v-if="!loading">
    <section v-else-if="record">
      <div>
        <span><strong>Execution Code: </strong></span>
        <span>{{ executedInstance?.execution_code }} </span>
      </div>

      <div>
        <span><strong>Required Inputs: </strong></span>
        <span>{{ executedInstance?.required_inputs }} </span>
      </div>

      <div>
        <span><strong>Contract Output Sig: </strong></span>
        <span>{{ executedInstance?.contract_output_sig }} </span>
      </div>

      <div></div>
    </section>
    <div class="alert" v-else>
      The requested executed instance was not found.
    </div>
  </div>
  <progress v-else></progress>
  <div class="alert" v-if="error">Error: {error.message}</div>
</template>

<script lang="ts">
import { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { ComputedRef, inject } from "vue";
import { ExecutedInstance } from "./types";

export default {
  props: {
    executedInstanceHash: {
      type: Object,
      required: true,
    },
  },
  data(): {
    record: Record | undefined;
    loading: boolean;
    error: HolochainError | undefined;
  } {
    return {
      record: undefined,
      loading: true,
      error: undefined,
    };
  },
  computed: {
    executedInstance() {
      if (!this.record) return;
      return decode((this.record.entry as any).Present.entry) as ExecutedInstance;
    },
  },
  async mounted() {
    if (this.executedInstanceHash === undefined) {
      throw new Error(`The executedInstanceHash input is required for the ExecutedInstanceDetail element`);
    }
    await this.fetchExecutedInstance();
  },
  methods: {
    async fetchExecutedInstance() {
      try {
        this.loading = true;
        this.record = await this.client.callZome({
          cap_secret: null,
          role_name: "scwr",
          zome_name: "scwr",
          fn_name: "get_executed_instance",
          payload: this.executedInstanceHash,
        });
      } catch (e) {
        this.error = e as HolochainError;
      } finally {
        this.loading = false;
      }
    },
  },
  emits: ["executed-instance-deleted"],
  setup() {
    const client = (inject("client") as ComputedRef<AppClient>).value;
    return { client };
  },
};
</script>
