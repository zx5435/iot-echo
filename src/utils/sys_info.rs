extern crate sysinfo;

use sysinfo::System;

pub fn get_cpu_mem() -> (f32, f32) {
    let mut system = System::new_all();
    system.refresh_all();

    let mem = system.used_memory() as f32 / system.total_memory() as f32 * 100.0;
    let cpu = system.global_cpu_usage();

    // println!("cpu {} , mem {}", cpu, mem);
    (cpu, mem)
}

mod tests {
    #[test]
    fn test_get_cpu_mem() {
        use self::super::*;
        use std::collections::HashMap;

        let (cpu, mem) = get_cpu_mem();

        let mut map = HashMap::new();
        map.insert("cpu", cpu);
        map.insert("mem", mem);

        let ret = serde_json::to_string(&map).unwrap();
        println!("{}", ret);
    }
}
