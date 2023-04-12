use crate::{TimeZone, TimeZoneData};

#[derive(Debug)]
pub(crate) enum Item {
    Module {
        name: String,
        children: Vec<Item>,
    },
    Struct {
        name: String,
        full_name: String,
        canonical_name_of_link: Option<String>,
    },
}

pub(crate) fn generate_items<'a>(time_zones: impl Iterator<Item = &'a TimeZone>) -> Item {
    let mut root = Item::Module {
        name: "time_zones".to_owned(),
        children: Vec::new(),
    };

    for tz in time_zones {
        let canonical_name_of_link = match &tz.data {
            TimeZoneData::Link(canonical_name) => Some(canonical_name.clone()),
            TimeZoneData::Canonical(_) => None,
        };
        insert_zone(&mut root, &tz.name, &tz.name, canonical_name_of_link);
    }

    root
}

fn insert_zone(
    root: &mut Item,
    name: &str,
    original_name: &str,
    canonical_name_of_link: Option<String>,
) {
    let children = match root {
        Item::Module {
            ref mut children, ..
        } => children,
        Item::Struct { .. } => panic!("called `insert_zone` on a struct"),
    };

    match name.split_once('/') {
        None => {
            children.push(Item::Struct {
                name: name.to_owned(),
                full_name: original_name.to_owned(),
                canonical_name_of_link,
            });
        }
        Some((mod_name, rest)) => {
            let module = children
                .iter_mut()
                .find(|item| matches!(item, Item::Module { name, .. } if name == mod_name));
            let module = match module {
                Some(module) => module,
                None => {
                    let module = Item::Module {
                        name: mod_name.to_owned(),
                        children: Vec::new(),
                    };
                    children.push(module);
                    children.last_mut().expect("just pushed an element")
                }
            };

            insert_zone(module, rest, original_name, canonical_name_of_link);
        }
    }
}
