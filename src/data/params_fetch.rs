use std::collections::HashMap;
use crate::model::{load_params, ParamsYml};
use indexmap::IndexMap;
use serde_json::json;

fn get_sn_all(sn: String, suffix: Option<String>) -> String {
    match suffix {
        Some(suffix) => format!("{}_{}", sn, suffix),
        _ => sn,
    }
}

pub fn fetch_params(cfg: ParamsYml) {
    let sn_all = "test".to_string();
    println!("point num={}", cfg.attributes.len());

    let mut map2 = HashMap::new();

    for point in cfg.attributes {
        let sn = get_sn_all(sn_all.clone(), point.namespace);

        let mut map1 = IndexMap::new();
        map1.insert("ts".to_string(), json!("qwe".to_string()));
        map1.insert("sn".to_string(), json!(sn.clone()));
        map1.insert("qwe".to_string(), json!(456.5));

        let s = serde_json::to_string(&map1);
        println!("{}", s.unwrap());
        map2.insert(sn, map1);
    }

    for point in cfg.attributes {
        let sn = get_sn_all(sn_all.clone(), point.namespace);
        map2.get(sn);
    }

    println!("map2 = {:#?}", &map2);
}

mod tests {
    use super::*;

    #[test]
    fn test_load_params_cfg() {
        let cfg = load_params();
        fetch_params(cfg)
    }
}
