use crate::prelude::*;


fn test_serialize_deserialize_quick_bin<T>(value: &T) where T: IoLoad + IoSave + PartialEq + Debug
{
    //println!("   value: {value:?}");
    let format = value.to_quick_bin().unwrap();
    //println!("   ron: {}", value.to_ron().unwrap());
    //println!("   bin: {format:?}");
    //println!();

    // Not a self describing format
    let from_format = T::from_quick_bin_buf(&format).unwrap();
    //println!("=>  ron: {}", from_format.to_ron().unwrap());
    //println!("=>  bin: {:?}", from_format.to_quick_bin());
    //println!("ron2value: {from_format:?}");
    assert_eq!(*value, from_format);
    //println!();
    //println!();
    //println!();
    //println!();
    //println!();
}

fn test_serialize_deserialize_ron<T>(value: &T) where T: IoLoad + IoSave + PartialEq + Debug
{
    let format = value.to_ron().unwrap();
    let from_format = T::from_ron(&format).unwrap();
    assert_eq!(*value, from_format);
}

fn serde_test<T>(value: &T) where T: IoLoad + IoSave + PartialEq + Debug
{
    test_serialize_deserialize_quick_bin(value);
    test_serialize_deserialize_ron(value);
}

#[test]
fn serialize_integer()
{
    serde_test(&42);
    serde_test(&128usize); // always serialized / deserialized as u64
    serde_test(&-96isize); // always serialized / deserialized as i64
}

#[test]
fn serialize_char()
{
    for c in "x\0\r\n .<>{}[]()".chars()
    {
        serde_test(&c);
    }
}

#[test]
fn serialize_string()
{
    serde_test(&"hello world!".to_owned());
    serde_test(&"".to_owned());
    serde_test(&"abc".to_owned());

    for sep in ",;.:/\0\r\n".chars()
    {
        serde_test(&sep.to_string());
        serde_test(&format!("abc{sep}"));
        serde_test(&format!("abc{sep}"));
        serde_test(&format!("abc{sep}{sep}"));
        serde_test(&format!("{sep}{sep}abc"));
        serde_test(&format!("{sep}abc{sep}{sep}"));
    }

    let all_ascii: String = (0u8..=127).map(|b| b as char).collect();
    serde_test(&all_ascii);

    for (open, close) in [('{', '}'), ('[', ']'), ('<', '>'), ('(', ')')]
    {
        serde_test(&open.to_string());
        serde_test(&close.to_string());

        serde_test(&format!("{open}{close}"));
        serde_test(&format!("{open}{close}{close}"));
        serde_test(&format!("{close}{open}"));
        serde_test(&format!("{close}{close}{open}"));
        serde_test(&format!("{open}{open}"));
        serde_test(&format!("{close}{close}"));

        serde_test(&format!("{open}abc{close}"));
        serde_test(&format!("{close}abc{open}"));

        serde_test(&format!("{open}{all_ascii}{close}"));
        serde_test(&format!("{close}{all_ascii}{open}"));
    }
}

#[test]
fn serialize_vec()
{
    serde_test(&Vec::<u8>::new());
    serde_test(&vec![1,4,3,2]);
}

#[test]
fn serialize_hashmap()
{
    serde_test(&((0..5).map(|i| (i.to_string(), i)).to_hashmap()));
}

#[test]
fn serialize_hashset()
{
    serde_test(&((0..5).map(|i| i.to_string())).to_hashset());
}

#[test]
fn serialize_genvec()
{
    serde_test(&((0..1).map(|i| i.to_string())).to_genvec());
    let mut g = (0..3).map(|i| i.to_string()).to_genvec();
    g.remove_from_index(0);
    g.remove_from_index(1);
    serde_test(&g);
}

#[test]
fn serialize_multihashmap()
{
    let multihashmap = [(["1".to_owned(), "one".to_owned()], 1), (["2".to_owned(), "deux".to_owned()], 2)].to_multihashmap();
    serde_test(&multihashmap);
}

#[test]
fn serialize_fixed_size_vector()
{
    serde_test(&point1(0));
    serde_test(&point2(10, 20));
    serde_test(&point3(10, 20, -30));
    serde_test(&point4(10, 20, -30, -40));

    serde_test(&vec2(10.5, 20.25));
    serde_test(&vec1(-8.));
    serde_test(&vec3(10.5, 20.25, -5.75));
    serde_test(&vec4(10.5, 20.25, 5.75, -4.5));

    serde_test(&Color::ROSE);
    serde_test(&RgbaF32::ROSE);
    serde_test(&RgbaF64::ROSE);
    serde_test(&RgbaU8::ROSE);
    serde_test(&RgbaU16::ROSE);
    serde_test(&HslaF32::ROSE);
    serde_test(&HslaF64::ROSE);


    serde_test(&Rect1::SIZED_ONE);
    serde_test(&Rect2::SIZED_ONE);
    serde_test(&Rect3::SIZED_ONE);
    serde_test(&Rect4::SIZED_ONE);

    serde_test(&rect2i(0, 3, 2, 2));
}

#[test]
fn serialize_unit()
{
    serde_test(&0.degree());
    serde_test(&45.degree());

    serde_test(&0.s());
    serde_test(&60.s());
}


#[test]
fn serialize_grid()
{
    serde_test(&Grid2::from_fn(point([3, 4]), |x| x.sum_axis()));
    serde_test(&Image::from_fn(point([3, 4]), |x| RgbaU8::rgb(x.x as _, x.y as _ , 0)));
}