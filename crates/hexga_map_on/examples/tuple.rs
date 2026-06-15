use hexga_map_on::*;

trait Foo
{
    fn foo();
}

impl Foo for i32
{
    fn foo()
    {
        println!("foo from i32");
    }
}
impl Foo for bool
{
    fn foo()
    {
        println!("foo from bool");
    }
}

map_on_tuple!(
    (
        $(
            $len:literal => ( $( $idx:tt $typ:ident )+ )
        )*
    ) => {
        $(
            #[cfg_attr(docsrs, doc(fake_variadic))]
            impl<$( $typ: Foo ),+> Foo for ( $( $typ ),+ ,) {
                fn foo() {
                    println!("Foo from tuple size {}", $len);
                }
            }
        )*
    };
);

fn main()
{
    <i32>::foo();
    <bool>::foo();

    <(i32,)>::foo();
    <(i32, i32)>::foo();

    <(i32, bool)>::foo();
    <(i32, bool, i32)>::foo();
}
