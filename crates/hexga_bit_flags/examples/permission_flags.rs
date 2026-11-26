//! For the same permissions example with indexed bits, run
//! `cargo run -p hexga_bit_set --example permissions`.

use hexga_bit_flags::bit_index;

#[bit_index]
#[repr(u8)]
enum Permission
{
    Read,
    Write,
    Execute,
}

fn main()
{
    let mut permissions = Permission::Read | Permission::Write;
    assert!(permissions.contains_all(Permission::Read | Permission::Write));
    assert!(!permissions.contains(Permission::Execute));

    permissions.remove(Permission::Write).insert(Permission::Execute);

    assert_eq!(permissions.bits(), 0b0101);
    let enabled: Vec<_> = permissions.iter().collect();
    assert_eq!(enabled, [Permission::Read, Permission::Execute]);

    println!("Permissions: {:08b}", permissions.bits());
    println!("Enabled permissions: {enabled:?}");
}
