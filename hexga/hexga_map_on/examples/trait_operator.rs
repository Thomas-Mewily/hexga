use hexga_map_on::prelude::*;

#[derive(Debug)]
struct X(pub i32);

map_on_operator_binary!(
    (($trait_name: tt, $fn_name: tt)) => 
    {
        impl std::ops::$trait_name for X
        {
            type Output = X;
            fn $fn_name(self, rhs : Self) -> Self::Output { X(self.0.$fn_name(rhs.0)) }
        }
    }
);

fn main() 
{
    let x =      X(9) + X(3) * X(4) / X(2);
    assert_eq!(x.0,   9  +   3  *   4  /   2 );
    dbg!(&x);
}
