use std::collections::BTreeMap;
use anyhow::Result;
use log::debug;
use memflow::prelude::v1::*;
use pelite::pattern::Atom;

use super::patterns::all_patterns;

pub type SignatureMap = BTreeMap<String, BTreeMap<String, String>>;

fn pattern_to_string(pattern: &[Atom]) -> String {
    pattern
        .iter()
        .map(|atom| match atom {
            Atom::Byte(b) => format!("{:02X}", b),
            Atom::Save(_) => "?".to_string(),
            Atom::Skip(n) => {
                if *n == 1 {
                    "?".to_string()
                } else {
                    format!("?{{{}}}", n)
                }
            }
            _ => "?".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn signatures<P: Process + MemoryView>(_process: &mut P) -> Result<SignatureMap> {
    let mut result = BTreeMap::new();

    for pattern_set in all_patterns() {
        let mut module_sigs = BTreeMap::new();

        for (name, pattern) in pattern_set.patterns.entries() {
            let sig_str = pattern_to_string(pattern);

            debug!(
                "signature \"{}\" in {} = {}",
                name, pattern_set.name, sig_str
            );

            module_sigs.insert(name.to_string(), sig_str);
        }

        result.insert(pattern_set.name.to_string(), module_sigs);
    }

    Ok(result)
                }
