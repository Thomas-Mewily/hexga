use hexga::prelude::*;
use hexga_map_on::*;

struct X;

hexga_map_on::map_on!(((Add,add),(Sub,sub)),
    (($trait_name:tt, $fn_name:tt)) => {
    impl std::ops::$trait_name for X {
        type Output = X;
        fn $fn_name(self,rhs:Self)->Self::Output {
            X
        }
    }
});

/* 
map_on_operator_binary!(
    ($trait_name: tt, $fn_name: tt) => 
    {
        impl $trait_name for X
        {
            type Output = X;
            fn $fn_name(self, rhs : Self) -> Self::Output { X }
        }
    }
)


hexga_map_on::map_on!(((std::ops::Add,add),(std::ops::Sub,sub),),($trait_name:tt, $fn_name:tt) => {
    impl $trait_name for X {
        type Output = X;
        fn$fn_name(self,rhs:Self)->Self::Output {
            X
        }
    }
});
*/



fn main() 
{
    println!("Hello, world!");
}
