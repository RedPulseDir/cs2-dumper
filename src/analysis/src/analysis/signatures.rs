use std::collections::BTreeMap;
use anyhow::Result;
use log::debug;
use memflow::prelude::v1::*;
use pelite::pattern::Atom;
use pelite::pe64::{Pe, PeView};

pub type SignatureMap = BTreeMap<String, BTreeMap<String, String>>;

/// Конвертирует паттерн в строковое представление
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

pub fn signatures<P: Process + MemoryView>(process: &mut P) -> Result<SignatureMap> {
    use crate::analysis::offsets::{client, engine2, input_system, matchmaking, soundsystem};
    
    let modules: Vec<(&str, &phf::Map<&'static str, (&'static [Atom], _)>)> = vec![
        ("client.dll", &client::PATTERNS),
        ("engine2.dll", &engine2::PATTERNS),
        ("inputsystem.dll", &input_system::PATTERNS),
        ("matchmaking.dll", &matchmaking::PATTERNS),
        ("soundsystem.dll", &soundsystem::PATTERNS),
    ];

    let mut result = BTreeMap::new();

    for (module_name, patterns) in &modules {
        let module = match process.module_by_name(module_name) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let buf = process
            .read_raw(module.base, module.size as _)
            .data_part()?;

        let view = PeView::from_bytes(&buf)?;

        let mut module_sigs = BTreeMap::new();

        for (name, (pattern, _)) in patterns.entries() {
            let sig_str = pattern_to_string(pattern);
            
            debug!(
                "signature \"{}\" in {} = {}",
                name, module_name, sig_str
            );

            module_sigs.insert(name.to_string(), sig_str);
        }

        result.insert(module_name.to_string(), module_sigs);
    }

    Ok(result)
}
