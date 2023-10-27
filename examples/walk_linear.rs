extern crate hwloc2;

use hwloc2::Topology;

/// Walk the topology with an array style, from level 0 (always
/// the system level) to the lowest level (always the proc level).
fn main() {
    let topo = Topology::new().unwrap();

    for i in 0..topo.depth() {
        println!("*** Objects at level {}", i);

        for (idx, object) in topo.objects_at_depth(i).iter().enumerate() {
            println!("{}: {}", idx, object);
        }
    }
}
