use crate::prelude::*;

fn test_serialize_deserialize<T>(value: &T) where T: IoLoad + IoSave + PartialEq + Debug
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

#[test]
fn serialize_integer()
{
    test_serialize_deserialize(&42);
    test_serialize_deserialize(&128usize); // always serialized / deserialized as u64
    test_serialize_deserialize(&-96isize); // always serialized / deserialized as i64
}

#[test]
fn serialize_char()
{
    test_serialize_deserialize(&'x');
    test_serialize_deserialize(&'\0');
    test_serialize_deserialize(&'\n');
}

#[test]
fn serialize_string()
{
    test_serialize_deserialize(&"hello world!".to_owned());
    test_serialize_deserialize(&"".to_owned());
    test_serialize_deserialize(&"abc".to_owned());

    for sep in ",;.:/\0\r\n".chars()
    {
        test_serialize_deserialize(&sep.to_string());
        test_serialize_deserialize(&format!("abc{sep}"));
        test_serialize_deserialize(&format!("abc{sep}"));
        test_serialize_deserialize(&format!("abc{sep}{sep}"));
        test_serialize_deserialize(&format!("{sep}{sep}abc"));
        test_serialize_deserialize(&format!("{sep}abc{sep}{sep}"));
    }

    let all_ascii: String = (0u8..=127).map(|b| b as char).collect();
    test_serialize_deserialize(&all_ascii);

    for (open, close) in [('{', '}'), ('[', ']'), ('<', '>'), ('(', ')')]
    {
        test_serialize_deserialize(&open.to_string());
        test_serialize_deserialize(&close.to_string());

        test_serialize_deserialize(&format!("{open}{close}"));
        test_serialize_deserialize(&format!("{open}{close}{close}"));
        test_serialize_deserialize(&format!("{close}{open}"));
        test_serialize_deserialize(&format!("{close}{close}{open}"));
        test_serialize_deserialize(&format!("{open}{open}"));
        test_serialize_deserialize(&format!("{close}{close}"));

        test_serialize_deserialize(&format!("{open}abc{close}"));
        test_serialize_deserialize(&format!("{close}abc{open}"));

        test_serialize_deserialize(&format!("{open}{all_ascii}{close}"));
        test_serialize_deserialize(&format!("{close}{all_ascii}{open}"));
    }
}

#[test]
fn serialize_vec()
{
    test_serialize_deserialize(&Vec::<u8>::new());
    test_serialize_deserialize(&vec![1,4,3,2]);
}

#[test]
fn serialize_hashmap()
{
    test_serialize_deserialize(&((0..5).map(|i| (i.to_string(), i)).to_hashmap()));
}

#[test]
fn serialize_hashset()
{
    test_serialize_deserialize(&((0..5).map(|i| i.to_string())).to_hashset());
}

#[test]
fn serialize_genvec()
{
    test_serialize_deserialize(&((0..1).map(|i| i.to_string())).to_genvec());
    let mut g = (0..3).map(|i| i.to_string()).to_genvec();
    g.remove_from_index(0);
    g.remove_from_index(1);
    test_serialize_deserialize(&g);
}

#[test]
fn serialize_multihashmap()
{
    let multihashmap = [(["1".to_owned(), "one".to_owned()], 1), (["2".to_owned(), "deux".to_owned()], 2)].to_multihashmap();
    test_serialize_deserialize(&multihashmap);
}

#[test]
fn serialize_fixed_size_vector()
{
    test_serialize_deserialize(&point1(0));
    test_serialize_deserialize(&point2(10, 20));
    test_serialize_deserialize(&point3(10, 20, -30));
    test_serialize_deserialize(&point4(10, 20, -30, -40));

    test_serialize_deserialize(&vec2(10.5, 20.25));
    test_serialize_deserialize(&vec1(-8.));
    test_serialize_deserialize(&vec3(10.5, 20.25, -5.75));
    test_serialize_deserialize(&vec4(10.5, 20.25, 5.75, -4.5));

    test_serialize_deserialize(&Color::ROSE);
    test_serialize_deserialize(&RgbaF32::ROSE);
    test_serialize_deserialize(&RgbaF64::ROSE);
    test_serialize_deserialize(&RgbaU8::ROSE);
    test_serialize_deserialize(&RgbaU16::ROSE);
    test_serialize_deserialize(&HslaF32::ROSE);
    test_serialize_deserialize(&HslaF64::ROSE);

    test_serialize_deserialize(&Rect1::SIZED_ONE);
    test_serialize_deserialize(&Rect2::SIZED_ONE);
    test_serialize_deserialize(&Rect3::SIZED_ONE);
    test_serialize_deserialize(&Rect4::SIZED_ONE);

    test_serialize_deserialize(&rect2i(0, 3, 2, 2));
}

#[test]
fn serialize_unit()
{
    test_serialize_deserialize(&0.degree());
    test_serialize_deserialize(&45.degree());

    test_serialize_deserialize(&0.s());
    test_serialize_deserialize(&60.s());
}

