//! For the same permissions example with named flags, run
//! `cargo run -p hexga_bit_flags --example permission_flags`.

use hexga_bit_set::prelude::*;
use hexga_core::prelude::*;

const READ: BitIndex = 0;
const WRITE: BitIndex = 1;
const EXECUTE: BitIndex = 2;

fn main() -> Result<(), ManyMutError>
{
    let mut permissions = BitSet::from(0b0011u8);
    assert!(permissions[READ]);
    assert!(permissions[WRITE]);
    assert!(!permissions[EXECUTE]);

    {
        // Borrow two distinct bits from the same byte.
        let [mut write, mut execute] = permissions.try_get_many_mut([WRITE, EXECUTE])?;
        *write = false;
        *execute = true;
    } // Both guards write their changes back when dropped.

    assert_eq!(permissions.0, 0b0101);
    assert_eq!(permissions.len(), 2);
    assert_eq!(permissions.capacity(), 8);
    assert!(matches!(permissions.try_get_many_mut([READ, READ]), Err(ManyMutError::OverlappingIndices)));

    let enabled: Vec<_> = permissions.iter_ones().collect();
    assert_eq!(enabled, [READ, EXECUTE]);
    println!("Permissions: {:08b}", permissions.0);
    println!("Enabled bit indices: {enabled:?}");

    Ok(())
}
