extern crate hwloc2;

use hwloc2::{Topology, ObjectType};

/// Compute the amount of cache that the first logical processor
/// has above it.
fn main() {
    let topo = Topology::new().unwrap();

    let pu = topo.objects_with_type(&ObjectType::PU).unwrap()[0];

    let mut parent = pu.parent();
    let mut levels = 0;
    let mut size = 0;

    while let Some(p) = parent {
        match p.object_type() {
            ObjectType::L1Cache | ObjectType::L2Cache | ObjectType::L3Cache | ObjectType::L4Cache | ObjectType::L5Cache => {
                levels += 1;
                // This should actually be size(), but there is a (compiler) bug? with the c-ffi unions
                size += p.cache_attributes().unwrap().size;
            },
            _ => {},
        }
        parent = p.parent();
    }

    println!("*** Logical processor 0 has {} caches totalling {} KB",
             levels,
             size / 1024);
}
