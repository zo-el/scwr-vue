<template>
  <div>
    <h3>Create Executed Instance</h3>

    <div>
      <label for="Execution Code">Execution Code</label>
      <textarea
        name="Execution Code"
        v-model="executionCode"
        required
      ></textarea>
    </div>
    <div>
      <label for="Required Inputs">Required Inputs</label>
      <textarea
        name="Required Inputs"
        v-model="requiredInputs"
        required
      ></textarea>
    </div>
    <div>
      <label for="Contract Output Sig">Contract Output Sig</label>
      <textarea
        name="Contract Output Sig"
        v-model="contractOutputSig"
        required
      ></textarea>
    </div>

    <button
      :disabled="!isExecutedInstanceValid"
      @click="createExecutedInstance"
    >
      Create ExecutedInstance
    </button>
  </div>
</template>

<script lang="ts">
import { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, Record } from "@holochain/client";
import { ComputedRef, inject } from "vue";
import { ExecutedInstance } from "./types";

export default {
  data(): {
    executionCode: string;
    requiredInputs: string;
    contractOutputSig: string;
  } {
    return {
      executionCode: "",
      requiredInputs: "",
      contractOutputSig: "",
    };
  },
  computed: {
    isExecutedInstanceValid() {
      return true && this.executionCode !== "" && this.requiredInputs !== "" && this.contractOutputSig !== "";
    },
  },
  mounted() {
  },
  methods: {
    async createExecutedInstance() {
      const executedInstance: ExecutedInstance = {
        execution_code: this.executionCode!,
        required_inputs: this.requiredInputs!,
        contract_output_sig: this.contractOutputSig!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: "scwr",
          zome_name: "scwr",
          fn_name: "create_executed_instance",
          payload: executedInstance,
        });
        this.$emit("executed-instance-created", record.signed_action.hashed.hash);
      } catch (e: any) {
        alert(e);
      }
    },
  },
  emits: ["executed-instance-created"],
  setup() {
    const client = (inject("client") as ComputedRef<AppClient>).value;
    return { client };
  },
};
</script>
