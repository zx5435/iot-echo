use std::any::Any;
use std::collections::HashMap;
use crate::model::{load_params, ParamsYml};

// struct M1Val {
//     ts: usize,
//     sn: String,
//     values: HashMap<String, dyn Any>,
// }

fn get_sn_all(sn: String, suffix: Option<String>) -> String {
    match suffix {
        Some(suffix) => format!("{}_{}", sn, suffix),
        _ => sn,
    }
}

pub fn fetch_params(cfg: ParamsYml) {
    let sn = "test".to_string();
    println!("point num={}", cfg.attributes.len());

    for point in cfg.attributes {
        let sno = get_sn_all(sn.clone(), point.namespace);

        let mut map1: HashMap<String, Box<dyn Any>> = HashMap::new();
        map1.insert("ts".to_string(), Box::new("qwe".to_string()));
        map1.insert("sn".to_string(), Box::new(sno.clone()));
        map1.insert("qwe".to_string(), Box::new(456));

        println!("{} {} {:#?}", sno, point.name, map1);
        println!("{:#?}", map1.get("qwe").unwrap() as usize);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_load_params_cfg() {
        let cfg = load_params();
        fetch_params(cfg)
    }
}
