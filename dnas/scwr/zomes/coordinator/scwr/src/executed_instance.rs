use hdk::prelude::*;
use scwr_integrity::*;

#[hdk_extern]
pub fn create_executed_instance(executed_instance: ExecutedInstance) -> ExternResult<Record> {
    let executed_instance_hash =
        create_entry(&EntryTypes::ExecutedInstance(executed_instance.clone()))?;
    let record = get(executed_instance_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created ExecutedInstance".to_string())
    ))?;
    let path = Path::from("all_instances");
    create_link(
        path.path_entry_hash()?,
        executed_instance_hash.clone(),
        LinkTypes::AllInstances,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_executed_instance(executed_instance_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(executed_instance_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}
