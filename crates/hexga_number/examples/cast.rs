use hexga_number::{CastInto, CastRangeInto};

fn cast<From, To>(from: From) -> To
where
    From: CastInto<To>,
{
    from.cast_into()
}

fn cast_range<To, From>(from: From) -> To
where
    From: CastRangeInto<To>,
{
    from.cast_range_into()
}

fn main() {
    dbg!(cast::<_, u8>(-1f32));
    dbg!(cast::<_, u8>(-0.5f32));
    dbg!(cast::<_, u8>(0f32));
    dbg!(cast::<_, u8>(0.5f32));
    dbg!(cast::<_, u8>(1f32));

    let value = 0.5f32;
    assert_eq!(cast::<_, u8>(&value), cast::<_, u8>(value));
    assert_eq!(cast_range::<u8, _>(&value), cast_range::<u8, _>(value));

    let flags = [false, true];
    assert_eq!(cast::<_, [u8; 2]>(&flags), [0, 1]);
    assert_eq!(cast_range::<[u8; 2], _>(&flags), [0, 255]);
}
