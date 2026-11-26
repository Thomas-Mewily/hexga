use hexga::prelude::*;

#[test]
fn grid_mapping_and_cropping_keep_the_allocator() {
    let allocator = AllocSystem;
    let source = Grid::<_, 2, _>::from_fn_in(vec2i(2, 3), |p| p.x + 10 * p.y, &allocator);
    let cropped = source.crop(rect2i(1, 0, 1, 3)).unwrap();
    let mapped = cropped.map_into(|value| value as i64 * 2);
    assert!(std::ptr::eq(*mapped.allocator(), &allocator));
    assert_eq!(mapped.into_values().as_slice(), &[2, 22, 42]);
}

#[test]
fn grid_and_image_conversion_reuses_the_storage() {
    let allocator = AllocSystem;
    let size = vec2i(2, 3);
    let grid = Grid::<_, 2, _>::from_fn_in(size, |p| p.x + 10 * p.y, &allocator);
    let storage = grid.values().as_ptr();
    let copied_image = grid.view().to_image();

    let image: ImageOf<_, _> = grid.into();
    assert!(std::ptr::eq(*image.allocator(), &allocator));
    assert_eq!(image.pixels().as_ptr(), storage);
    assert_eq!(image, copied_image);
    assert_eq!(image[vec2i(0, 0)], 0);
    assert_eq!(image[vec2i(1, 2)], 21);

    let grid = image.to_grid();
    assert_eq!(grid.values().as_ptr(), storage);
    assert_eq!(grid.values(), &[0, 1, 10, 11, 20, 21]);
}

#[test]
fn generational_sequence_uses_the_allocator_across_removal_and_growth() {
    let allocator = AllocSystem;
    let mut sequence = GenVec::<_, _>::with_capacity_and_param(4, &allocator);
    assert!(Capacity::capacity(&sequence) >= 4);
    sequence.extend([42, 43]);
    let first = sequence.iter().next().unwrap().0;
    assert_eq!(sequence.remove(first), Some(42));
    let replacement = sequence.insert(44);
    assert_ne!(replacement, first);
    assert_eq!(sequence.get(replacement), Some(&44));
    assert_eq!(sequence.get(first), None);

    Reserve::reserve(&mut sequence, 16);
    assert!(Capacity::capacity(&sequence) >= 16);
}
