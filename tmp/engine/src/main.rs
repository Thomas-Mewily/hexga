#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::prelude::*;

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
    hexga_engine::run(|| MyApp{});
    //MyApp::run(|| MyApp{});
    //assert!(matches!(MyOption::<std::num::NonZeroU8>::zeroed(), MyOption::None));
    //AlwaysBitZero::<std::num::NonZeroU8>::zeroed();

    //let b = Bar::zeroed();
    //dbg!(b);
}
