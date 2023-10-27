extern crate hwloc2;

use hwloc2::{Topology, ObjectType, TypeDepthError};

/// Prints the number of packages.
fn main() {
    let topo = Topology::new().unwrap();

    let package_depth = topo.depth_for_type(&ObjectType::Package);
    match package_depth {
        Ok(depth) => println!("*** {} package(s)", topo.size_at_depth(depth)),
        Err(TypeDepthError::TypeDepthUnknown) => println!("*** The number of packages is unknown"),
        Err(e) => println!("Unknown Error: {:?}", e),
    }
}
