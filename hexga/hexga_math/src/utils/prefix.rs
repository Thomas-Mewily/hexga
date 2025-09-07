//! Contains some common prefix, such as `[Kilo]`, `[Milli]`, `[Tera]`
use super::*;

pub mod prelude
{
    pub use super::{PrefixPeta,PrefixQuadrillion,PrefixTera,PrefixTrillion,PrefixGiga,PrefixBillion,PrefixMega,PrefixMillion,PrefixKilo,PrefixThousand,PrefixHecto,PrefixHundred,PrefixDeca,PrefixDecade,PrefixDeci,PrefixCenti,PrefixMilli,PrefixMicro,PrefixNano,PrefixPico,PrefixFemto};
}

// TODO: split each of them in 2 trait. ConstPrefixPeta, PrefixPeta (same with constant number)

/// 10^15 = 1_000_000_000_000_000
pub trait PrefixPeta : Sized 
{ 
    /// 10^15 = 1_000_000_000_000_000
    const PETA : Self;
    /// multiply by 10^15 = 1_000_000_000_000_000
    fn peta(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::PETA  } 
}
impl PrefixPeta for f64 { const PETA : Self = 1_000_000_000_000_000.; }
impl PrefixPeta for f32 { const PETA : Self = 1_000_000_000_000_000.; }
impl PrefixPeta for u64 { const PETA : Self = 1_000_000_000_000_000 ; }
impl PrefixPeta for i64 { const PETA : Self = 1_000_000_000_000_000 ; }


/// 10^15 = 1_000_000_000_000_000
pub trait PrefixQuadrillion : Sized 
{ 
    /// 10^15 = 1_000_000_000_000_000
    const QUADRILLION : Self;
    /// multiply by 10^15 = 1_000_000_000_000_000
    fn quadrillion(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::QUADRILLION  } 
}
impl PrefixQuadrillion for f64 { const QUADRILLION : Self = 1_000_000_000_000_000.; }
impl PrefixQuadrillion for f32 { const QUADRILLION : Self = 1_000_000_000_000_000.; }
impl PrefixQuadrillion for u64 { const QUADRILLION : Self = 1_000_000_000_000_000 ; }
impl PrefixQuadrillion for i64 { const QUADRILLION : Self = 1_000_000_000_000_000 ; }



/// 10^12 = 1_000_000_000_000
pub trait PrefixTera : Sized 
{ 
    /// 10^12 = 1_000_000_000_000
    const TERA : Self;
    /// multiply by 10^12 = 1_000_000_000_000
    fn tera(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::TERA  } 
}
impl PrefixTera for f64 { const TERA : Self = 1_000_000_000_000.; }
impl PrefixTera for f32 { const TERA : Self = 1_000_000_000_000.; }
impl PrefixTera for u64 { const TERA : Self = 1_000_000_000_000 ; }
impl PrefixTera for i64 { const TERA : Self = 1_000_000_000_000 ; }


/// 10^12 = 1_000_000_000_000
pub trait PrefixTrillion : Sized 
{ 
    /// 10^12 = 1_000_000_000_000
    const TRILLION : Self;
    /// multiply by 10^12 = 1_000_000_000_000
    fn trillion(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::TRILLION  } 
}
impl PrefixTrillion for f64 { const TRILLION : Self = 1_000_000_000_000.; }
impl PrefixTrillion for f32 { const TRILLION : Self = 1_000_000_000_000.; }
impl PrefixTrillion for u64 { const TRILLION : Self = 1_000_000_000_000 ; }
impl PrefixTrillion for i64 { const TRILLION : Self = 1_000_000_000_000 ; }





/// 10^9 = 1_000_000_000
pub trait PrefixGiga : Sized 
{ 
    /// 10^9 = 1_000_000_000
    const GIGA : Self;
    /// multiply by 10^9 = 1_000_000_000
    fn giga(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::GIGA  } 
}
impl PrefixGiga for usize { const GIGA : Self = 1_000_000_000 ; }
impl PrefixGiga for isize { const GIGA : Self = 1_000_000_000 ; }
impl PrefixGiga for f64   { const GIGA : Self = 1_000_000_000.; }
impl PrefixGiga for f32   { const GIGA : Self = 1_000_000_000.; }
impl PrefixGiga for u64   { const GIGA : Self = 1_000_000_000 ; }
impl PrefixGiga for i64   { const GIGA : Self = 1_000_000_000 ; }
impl PrefixGiga for u32   { const GIGA : Self = 1_000_000_000 ; }
impl PrefixGiga for i32   { const GIGA : Self = 1_000_000_000 ; }


/// 10^9 = 1_000_000_000
pub trait PrefixBillion : Sized 
{ 
    /// 10^9 = 1_000_000_000
    const BILLION : Self;
    /// multiply by 10^9 = 1_000_000_000
    fn billion(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::BILLION  } 
}
impl PrefixBillion for usize { const BILLION : Self = 1_000_000_000 ; }
impl PrefixBillion for isize { const BILLION : Self = 1_000_000_000 ; }
impl PrefixBillion for f64   { const BILLION : Self = 1_000_000_000.; }
impl PrefixBillion for f32   { const BILLION : Self = 1_000_000_000.; }
impl PrefixBillion for u64   { const BILLION : Self = 1_000_000_000 ; }
impl PrefixBillion for i64   { const BILLION : Self = 1_000_000_000 ; }
impl PrefixBillion for u32   { const BILLION : Self = 1_000_000_000 ; }
impl PrefixBillion for i32   { const BILLION : Self = 1_000_000_000 ; }




/// 10^6 = 1_000_000
pub trait PrefixMega : Sized 
{ 
    /// 10^6 = 1_000_000
    const MEGA : Self;
    /// multiply by 10^6 = 1_000_000
    fn mega(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::MEGA  } 
}
impl PrefixMega for usize { const MEGA : Self = 1_000_000 ; }
impl PrefixMega for isize { const MEGA : Self = 1_000_000 ; }
impl PrefixMega for f64   { const MEGA : Self = 1_000_000.; }
impl PrefixMega for f32   { const MEGA : Self = 1_000_000.; }
impl PrefixMega for u64   { const MEGA : Self = 1_000_000 ; }
impl PrefixMega for i64   { const MEGA : Self = 1_000_000 ; }
impl PrefixMega for u32   { const MEGA : Self = 1_000_000 ; }
impl PrefixMega for i32   { const MEGA : Self = 1_000_000 ; }



/// 10^6 = 1_000_000
pub trait PrefixMillion : Sized 
{ 
    /// 10^6 = 1_000_000
    const MILLION : Self;
    /// multiply by 10^6 = 1_000_000
    fn million(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::MILLION  } 
}
impl PrefixMillion for usize { const MILLION : Self = 1_000_000 ; }
impl PrefixMillion for isize { const MILLION : Self = 1_000_000 ; }
impl PrefixMillion for f64   { const MILLION : Self = 1_000_000.; }
impl PrefixMillion for f32   { const MILLION : Self = 1_000_000.; }
impl PrefixMillion for u64   { const MILLION : Self = 1_000_000 ; }
impl PrefixMillion for i64   { const MILLION : Self = 1_000_000 ; }
impl PrefixMillion for u32   { const MILLION : Self = 1_000_000 ; }
impl PrefixMillion for i32   { const MILLION : Self = 1_000_000 ; }



/// 10^3 = 1_000
pub trait PrefixKilo : Sized 
{ 
    /// 10^3 = 1_000
    const KILO : Self;
    /// multiply by 10^3 = 1_000
    fn kilo(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::KILO  } 
}
impl PrefixKilo for usize { const KILO : Self = 1_000 ; }
impl PrefixKilo for isize { const KILO : Self = 1_000 ; }
impl PrefixKilo for f64   { const KILO : Self = 1_000.; }
impl PrefixKilo for f32   { const KILO : Self = 1_000.; }
impl PrefixKilo for u64   { const KILO : Self = 1_000 ; }
impl PrefixKilo for i64   { const KILO : Self = 1_000 ; }
impl PrefixKilo for u32   { const KILO : Self = 1_000 ; }
impl PrefixKilo for i32   { const KILO : Self = 1_000 ; }
impl PrefixKilo for u16   { const KILO : Self = 1_000 ; }
impl PrefixKilo for i16   { const KILO : Self = 1_000 ; }


/// 10^3 = 1_000
pub trait PrefixThousand : Sized 
{ 
    /// 10^3 = 1_000
    const THOUSAND : Self;
    /// multiply by 10^3 = 1_000
    fn thousand(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::THOUSAND  } 
}
impl PrefixThousand for usize { const THOUSAND : Self = 1_000 ; }
impl PrefixThousand for isize { const THOUSAND : Self = 1_000 ; }
impl PrefixThousand for f64   { const THOUSAND : Self = 1_000.; }
impl PrefixThousand for f32   { const THOUSAND : Self = 1_000.; }
impl PrefixThousand for u64   { const THOUSAND : Self = 1_000 ; }
impl PrefixThousand for i64   { const THOUSAND : Self = 1_000 ; }
impl PrefixThousand for u32   { const THOUSAND : Self = 1_000 ; }
impl PrefixThousand for i32   { const THOUSAND : Self = 1_000 ; }
impl PrefixThousand for u16   { const THOUSAND : Self = 1_000 ; }
impl PrefixThousand for i16   { const THOUSAND : Self = 1_000 ; }



/// 10^2 = 100
pub trait PrefixHecto : Sized 
{ 
    /// 10^2 = 100
    const HECTO : Self;
    /// multiply by 10^2 = 100
    fn hecto(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::HECTO  } 
}

impl PrefixHecto for usize { const HECTO : Self = 100 ; }
impl PrefixHecto for isize { const HECTO : Self = 100 ; }
impl PrefixHecto for f64   { const HECTO : Self = 100.; }
impl PrefixHecto for f32   { const HECTO : Self = 100.; }
impl PrefixHecto for u64   { const HECTO : Self = 100 ; }
impl PrefixHecto for i64   { const HECTO : Self = 100 ; }
impl PrefixHecto for u32   { const HECTO : Self = 100 ; }
impl PrefixHecto for i32   { const HECTO : Self = 100 ; }
impl PrefixHecto for u16   { const HECTO : Self = 100 ; }
impl PrefixHecto for i16   { const HECTO : Self = 100 ; }
impl PrefixHecto for u8    { const HECTO : Self = 100 ; }
impl PrefixHecto for i8    { const HECTO : Self = 100 ; }



/// 10^2 = 100
pub trait PrefixHundred : Sized 
{ 
    /// 10^2 = 100
    const HUNDRED : Self;
    /// multiply by 10^2 = 100
    fn hundred(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::HUNDRED  } 
}

impl PrefixHundred for usize { const HUNDRED : Self = 100 ; }
impl PrefixHundred for isize { const HUNDRED : Self = 100 ; }
impl PrefixHundred for f64   { const HUNDRED : Self = 100.; }
impl PrefixHundred for f32   { const HUNDRED : Self = 100.; }
impl PrefixHundred for u64   { const HUNDRED : Self = 100 ; }
impl PrefixHundred for i64   { const HUNDRED : Self = 100 ; }
impl PrefixHundred for u32   { const HUNDRED : Self = 100 ; }
impl PrefixHundred for i32   { const HUNDRED : Self = 100 ; }
impl PrefixHundred for u16   { const HUNDRED : Self = 100 ; }
impl PrefixHundred for i16   { const HUNDRED : Self = 100 ; }
impl PrefixHundred for u8    { const HUNDRED : Self = 100 ; }
impl PrefixHundred for i8    { const HUNDRED : Self = 100 ; }



/// 10^1 = 10
pub trait PrefixDeca : Sized 
{ 
    /// 10^1 = 10
    const DECA : Self;
    /// multiply by 10^1 = 10
    fn deca(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::DECA  } 
}

impl PrefixDeca for usize { const DECA : Self = 10 ; }
impl PrefixDeca for isize { const DECA : Self = 10 ; }
impl PrefixDeca for f64   { const DECA : Self = 10.; }
impl PrefixDeca for f32   { const DECA : Self = 10.; }
impl PrefixDeca for u64   { const DECA : Self = 10 ; }
impl PrefixDeca for i64   { const DECA : Self = 10 ; }
impl PrefixDeca for u32   { const DECA : Self = 10 ; }
impl PrefixDeca for i32   { const DECA : Self = 10 ; }
impl PrefixDeca for u16   { const DECA : Self = 10 ; }
impl PrefixDeca for i16   { const DECA : Self = 10 ; }
impl PrefixDeca for u8    { const DECA : Self = 10 ; }
impl PrefixDeca for i8    { const DECA : Self = 10 ; }




/// 10^1 = 10
pub trait PrefixDecade : Sized 
{ 
    /// 10^1 = 10
    const DECADE : Self;
    /// multiply by 10^1 = 10
    fn decade(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::DECADE  } 
}

impl PrefixDecade for usize { const DECADE : Self = 10 ; }
impl PrefixDecade for isize { const DECADE : Self = 10 ; }
impl PrefixDecade for f64   { const DECADE : Self = 10.; }
impl PrefixDecade for f32   { const DECADE : Self = 10.; }
impl PrefixDecade for u64   { const DECADE : Self = 10 ; }
impl PrefixDecade for i64   { const DECADE : Self = 10 ; }
impl PrefixDecade for u32   { const DECADE : Self = 10 ; }
impl PrefixDecade for i32   { const DECADE : Self = 10 ; }
impl PrefixDecade for u16   { const DECADE : Self = 10 ; }
impl PrefixDecade for i16   { const DECADE : Self = 10 ; }
impl PrefixDecade for u8    { const DECADE : Self = 10 ; }
impl PrefixDecade for i8    { const DECADE : Self = 10 ; }



/// 10^-1 = 1/10
pub trait PrefixDeci : Sized 
{ 
    /// 10^-1 = 1/10
    const DECI : Self;
    /// multiply by 10^-1 = 1/10
    fn deci(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::DECI  } 
}
impl PrefixDeci for f64   { const DECI : Self = 1. / 10.; }
impl PrefixDeci for f32   { const DECI : Self = 1. / 10.; }



/// 10^-2 = 1/100
pub trait PrefixCenti : Sized 
{ 
    /// 10^-2 = 1/100
    const CENTI : Self;
    /// multiply by 10^-2 = 1/100
    fn centi(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::CENTI  } 
}
impl PrefixCenti for f64   { const CENTI : Self = 1. / 100.; }
impl PrefixCenti for f32   { const CENTI : Self = 1. / 100.; }



/// 10^-3 = 1/1_000
pub trait PrefixMilli : Sized 
{ 
    /// 10^-3 = 1/1_000
    const MILLI : Self;
    /// multiply by 10^-3 = 1/1_000
    fn milli(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::MILLI  } 
}
impl PrefixMilli for f64   { const MILLI : Self = 1. / 1_000.; }
impl PrefixMilli for f32   { const MILLI : Self = 1. / 1_000.; }


/// 10^-6 = 1/1_000_000
pub trait PrefixMicro : Sized 
{ 
    /// 10^-6 = 1/1_000_000
    const MICRO : Self;
    /// multiply by 10^-6 = 1/1_000_000
    fn micro(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::MICRO  } 
}
impl PrefixMicro for f64   { const MICRO : Self = 1. / 1_000_000.; }
impl PrefixMicro for f32   { const MICRO : Self = 1. / 1_000_000.; }


/// 10^-9 = 1/1_000_000_000
pub trait PrefixNano : Sized 
{ 
    /// 10^-9 = 1/1_000_000_000
    const NANO : Self;
    /// multiply by 10^-9 = 1/1_000_000_000
    fn nano(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::NANO  } 
}
impl PrefixNano for f64   { const NANO : Self = 1. / 1_000_000_000.; }
impl PrefixNano for f32   { const NANO : Self = 1. / 1_000_000_000.; }


/// 10^-12 = 1/1_000_000_000_000
pub trait PrefixPico : Sized 
{ 
    /// 10^-12 = 1/1_000_000_000_000
    const PICO : Self;
    /// multiply by 10^-12 = 1/1_000_000_000_000
    fn pico(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::PICO  } 
}
impl PrefixPico for f64   { const PICO : Self = 1. / 1_000_000_000_000.; }
impl PrefixPico for f32   { const PICO : Self = 1. / 1_000_000_000_000.; }


/// 10^-15 = 1/1_000_000_000_000_000
pub trait PrefixFemto : Sized 
{ 
    /// 10^-15 = 1/1_000_000_000_000_000
    const FEMTO : Self;
    /// multiply by 10^-15 = 1/1_000_000_000_000_000
    fn femto(self) -> Self where Self : Mul<Self,Output = Self> { self * Self::FEMTO  } 
}
impl PrefixFemto for f64   { const FEMTO : Self = 1. / 1_000_000_000_000_000.; }
impl PrefixFemto for f32   { const FEMTO : Self = 1. / 1_000_000_000_000_000.; }

#[cfg(test)]
mod prefix_test{
    use crate::*;

    #[test]
    fn test_prefix() 
    {
        assert_eq!(1.kilo(), 1000);
        assert_eq!([1,2].degree(), [1.degree(),2.degree()]);
        assert_eq!([1,2].s(), [1.s(),2.s()]);
        //assert_eq!([1,2].kilo(), [1.kilo(),2.kilo()]);
    }
}