use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::iter::FromIterator;
use crate::input::InputID;

type InputIDVal = usize;
type GroupID = usize;

type GroupData = HashMap<GroupID, Vec<InputIDVal>>;
type MembershipData = HashMap<InputID, GroupID>;

#[derive(Debug, Deserialize)]
struct InferenceObject(GroupData);

pub struct InferenceMap {
    groups: GroupData,
    memberships: MembershipData,
}

impl InferenceMap {
    pub fn new_from_json(path: &str) -> Self {
        let file = File::open(path)
            .expect("failed to open inference results");
        let results: serde_json::Value = serde_json::from_reader(file)
            .expect("failed to parse inference JSON");
        let inference: InferenceObject = serde_json::from_value(results).unwrap();

        let groups: GroupData = inference.0.into_iter()
                                .filter(|(_k, v)| v.len() > 0)
                                .collect();

        let mut memberships = MembershipData::new();
        for (key, values) in &groups {
            for value in values {
                memberships.insert(InputID::new(*value), *key);
            }
        }

        Self {
            groups,
            memberships,
        }
    }

    pub fn inputs_to_groups(&self, inputs: &mut dyn Iterator<Item = &InputID>)
            -> HashSet<Result<GroupID, InputID>> {
        let mut set: HashSet<Result<GroupID, InputID>> = HashSet::new();
        for id in inputs {
            if let Some(group) = self.memberships.get(id) {
                set.insert(Ok(*group));
            } else {
                set.insert(Err(*id));
            }
        }
        set
    }

    pub fn members(&self, group: GroupID) -> HashSet<InputID> {
        if let Some(inputs) = self.groups.get(&group) {
            inputs.into_iter().map(|val| {InputID::new(*val)}).collect()
        } else {
            panic!("Invalid group ID: {}", group)
        }
    }

    pub fn all_groups(&self) -> HashSet<GroupID> {
        HashSet::from_iter(self.groups.keys().map(|g| {*g}))
    }
}