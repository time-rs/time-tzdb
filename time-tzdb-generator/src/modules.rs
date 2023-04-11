use crate::{TimeZoneLinks, TimeZones};

#[derive(Debug)]
pub(crate) enum Item {
    Module { name: String, children: Vec<Item> },
    Struct { name: String, full_name: String },
}

pub(crate) fn generate_items(time_zones: &TimeZones, tz_links: &TimeZoneLinks) -> Item {
    let mut root = Item::Module {
        name: "time_zones".to_owned(),
        children: Vec::new(),
    };

    let zone_names = time_zones.iter().map(|tz| tz.0.as_str());
    let link_names = tz_links
        .values()
        .flat_map(|vec| vec.iter().map(|s| s.as_str()));

    for zone_name in zone_names.chain(link_names) {
        insert_zone(&mut root, zone_name, zone_name);
    }

    root
}

fn insert_zone(root: &mut Item, name: &str, original_name: &str) {
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

            insert_zone(module, rest, original_name);
        }
    }
}
