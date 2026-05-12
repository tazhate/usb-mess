use crate::model::Snapshot;

pub fn to_json(snap: &Snapshot) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(snap)?)
}
