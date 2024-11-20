import {
  ActionHash,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeDnaHash,
  fakeEntryHash,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { CallableCell } from "@holochain/tryorama";

export async function sampleExecutedInstance(cell: CallableCell, partialExecutedInstance = {}) {
  return {
    ...{
      execution_code: 'input + " World"',
      required_inputs: "Hello",
      contract_output_sig: "Hello World",
    },
    ...partialExecutedInstance,
  };
}

export async function createExecutedInstance(cell: CallableCell, executedInstance = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "scwr",
    fn_name: "create_executed_instance",
    payload: executedInstance || await sampleExecutedInstance(cell),
  });
}
