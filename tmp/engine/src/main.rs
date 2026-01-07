#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::{encoding::markup::ToJson, prelude::*};

/*
#[repr(C)]
#[derive(BitZero, Debug)]
struct Bar
{
    x: i32,
    y: i32,
}
*/

struct MyApp
{

}

impl Application for MyApp
{
    fn event(&mut self, ev: AppEvent) {
        dbg!(ev);
    }
}

use hexga_bit::{BitZero,BitZeroed};
use std::marker::PhantomData;

#[derive(Clone, BitZero)]
#[bitzero(bound = "")]
struct AlwaysBitZero<T> {
    a: PhantomData<T>,
}

fn main()
{
    //let json = Damage::<float>::ZERO.json

    hexga_engine::run_with_param(|| MyApp{}, AppParam::new().with_title("hello world"));
    //MyApp::run(|| MyApp{});
    //assert!(matches!(MyOption::<std::num::NonZeroU8>::zeroed(), MyOption::None));
    //AlwaysBitZero::<std::num::NonZeroU8>::zeroed();

    //let b = Bar::zeroed();
    //dbg!(b);
}
