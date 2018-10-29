extern crate crdts;

use crdts::{CmRDT, Map};

fn main() {
    let mut map = Map::new();
    let add_ctx = map
        .read()
        .derive_add_ctx(1);

    {
        let op = map.update(
            "bob",
            add_ctx,
            |set, c| set.add("is feeling O.K.", c)
        );
        map.apply(&op);
    }

    let map_on_2nd_device = map.clone();
    // the map on the 2nd devices adds to the set
    // under the "bob" key
    {
        let 2nd_device_add_ctx = map_on_2nd_device
            .read()
            .derive_add_ctx(2);
        let op = map_on_2nd_device.update(
            "bob",
            2nd_device_add_ctx,
            |set, c| set.add("is overwhelmed")
        );
        map_on_2nd_device.apply(&op);
    }
    // concurrently the map on the first device
    // remove 'bob'
    {
        let rm_ctx = map
            .get("bob")
            .derive_rm_ctx();
        map.rm("bob", rm_ctx);
    }

    // once these two devices synchronize...
    map.merge(&map_on_2nd_device);
    map_on_2nd_device.merge(&map);

    // we see that "bob" is present but the
    // set under bob only contains the changes
    // unseen by the first map

    let val = map
        .get("bob").val
        .map(|set| set.read().val);
    assert_eq!(
        val,
        Some(
            ["is overwhelmed"] // only entry left
                .into_iter()
                .collect()
        )
    );
}
