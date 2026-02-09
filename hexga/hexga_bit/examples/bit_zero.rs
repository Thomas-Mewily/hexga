use hexga_bit::prelude::*;

#[derive(Clone, BitZero, Debug)]
struct AlwaysBitZero<T>
{
    pub value: T
}

fn main()
{
    let v = AlwaysBitZero::<usize>::zeroed();
    dbg!(v.value);
}