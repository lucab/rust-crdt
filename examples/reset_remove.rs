extern crate crdts;

use crdts::{CvRDT, CmRDT, Map, Orswot};

fn main() {
    let mut friend_map: Map<String, Orswot<String, u8>, u8> = Map::new();
    let add_ctx = friend_map.len()
        .derive_add_ctx(1);

    {
        let op = friend_map.update(
            "bob",
            add_ctx,
            |set, ctx| set.add("janet", ctx)
        );
        map.apply(&op);
    }

    let mut map_on_device2 = map.clone();
    // the map on the 2nd devices adds to the set
    // under the "bob" key
    {
        let device2_add_ctx = map_on_device2
            .len()
            .derive_add_ctx(2);
        let op = map_on_device2.update(
            "bob",
            device2_add_ctx,
            |set, c| set.add("is overwhelmed", c)
        );
        map_on_device2.apply(&op);
    }
    // concurrently the map on the first device
    // remove 'bob'
    {
        let rm_ctx = map
            .get(&"bob".to_string())
            .derive_rm_ctx();
        map.rm("bob", rm_ctx);
    }

    // once these two devices synchronize...
    map.merge(&map_on_device2);
    map_on_device2.merge(&map);

    // we see that "bob" is present but the
    // set under bob only contains the changes
    // unseen by the first map

    let val = map
        .get(&"bob".to_string()).val
        .map(|set| set.read().val);
    assert_eq!(
        val,
        Some(
            // only one entry left
            vec!["is overwhelmed".to_string()]
                .into_iter()
                .collect()
        )
    );
}
