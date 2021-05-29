use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub want: Vec<String>,

    #[serde(default)]
    pub ignore: Vec<String>,

    #[serde(default, alias = "ignoreGroups")]
    pub ignore_groups: Vec<String>
}
