use anyhow::Result;
use serde::Deserialize;
use serde_json::{Value, from_value};
use std::{collections::HashSet, path::PathBuf};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let parsed_skills = parse_lua()?;
//     // let skills_edges = extract_edges(parsed_skills);
//     // let output_path = "output.json";
//     // std::fs::write(output_path, format!("{skills_edges:?}"))?;
//     graph::app_with_skills(&parsed_skills);
//     Ok(())
// }

// TODO: Remove this.
#[expect(unused, reason = "This function is not used anymore.")]
pub fn extract_edges(skills: Vec<Skill>) -> Vec<[u32; 2]> {
    let mut edges: HashSet<[u32; 2]> = HashSet::new();
    for skill in skills {
        if let Some(dep) = skill.dependencies {
            for d in &dep {
                edges.insert([skill.id, *d]);
            }
        }
    }

    edges.into_iter().collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Skill {
    pub id: u32,
    pub name: Option<String>,
    pub dependencies: Option<Vec<u32>>,
}

pub fn parse_lua() -> Result<Vec<Skill>> {
    let content = std::fs::read_to_string(PathBuf::from(
        "/Volumes/Dock/Dev/Rust/egui-xp/assets/nodes.json",
    ))?;
    let args: Value = serde_json::from_str(&content)?;
    let result = args
        .as_object()
        .expect("Could not convert Skill into object")
        .iter()
        .map(|(k, v)| {
            let raw: RawSkill =
                from_value(v.to_owned()).expect("Failed RawSkill to Owned Coversion");
            Skill {
                id: k.parse().unwrap_or(0),
                name: raw.name,
                dependencies: raw.dep,
            }
        })
        .collect();
    Ok(result)
}

fn from_str_vec_opt<'de, D>(deserializer: D) -> Result<Option<Vec<u32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    let Some(v) = value else { return Ok(None) };

    match v {
        serde_json::Value::Array(arr) if arr.is_empty() => Ok(None),
        serde_json::Value::Array(arr) => {
            let mut out = Vec::new();
            for item in arr {
                if let Some(s) = item.as_str() {
                    out.push(s.parse::<u32>().map_err(serde::de::Error::custom)?);
                }
            }
            Ok(Some(out))
        }
        serde_json::Value::String(s) => Ok(Some(vec![
            s.parse::<u32>().map_err(serde::de::Error::custom)?,
        ])),
        _ => Ok(None),
    }
}

#[expect(unused, reason = "RawSkill should not be necessary anymore.")]
#[derive(Deserialize, Debug, Default)]
struct RawSkill {
    name: Option<String>,
    #[serde(rename = "out", deserialize_with = "from_str_vec_opt", default)]
    dep: Option<Vec<u32>>,
}
