#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_map_on::*;

fn convert_range<T1 : Primitive, T2 : Primitive>(t1 : T1)
{
    match (T1::PRIMITIVE_NUMBER_TYPE, T2::PRIMITIVE_NUMBER_TYPE)
    {
        (NumberType::IntegerSigned, NumberType::IntegerSigned) => todo!(),
        (NumberType::IntegerSigned, NumberType::IntegerUnsigned) => todo!(),
        (NumberType::IntegerSigned, NumberType::Float) => todo!(),
        (NumberType::IntegerSigned, NumberType::Bool) => todo!(),
        (NumberType::IntegerUnsigned, NumberType::IntegerSigned) => todo!(),
        (NumberType::IntegerUnsigned, NumberType::IntegerUnsigned) => todo!(),
        (NumberType::IntegerUnsigned, NumberType::Float) => todo!(),
        (NumberType::IntegerUnsigned, NumberType::Bool) => todo!(),
        (NumberType::Float, NumberType::IntegerSigned) => todo!(),
        (NumberType::Float, NumberType::IntegerUnsigned) => todo!(),
        (NumberType::Float, NumberType::Float) => todo!(),
        (NumberType::Float, NumberType::Bool) => todo!(),
        (NumberType::Bool, NumberType::IntegerSigned) => todo!(),
        (NumberType::Bool, NumberType::IntegerUnsigned) => todo!(),
        (NumberType::Bool, NumberType::Float) => todo!(),
        (NumberType::Bool, NumberType::Bool) => todo!(),
    }
}

fn main() 
{

}
