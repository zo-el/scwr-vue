use hdk::prelude::*;
use scwr_integrity::*;

#[hdk_extern]
pub fn get_all_instances() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_instances");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllInstances)?.build(),
    )
}
