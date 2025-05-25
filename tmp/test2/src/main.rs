#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_map_on::*;

fn main() 
{
    dbg!(Color::WHITE);
    dbg!(Color::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorByte::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorMask::RED);

    dbg!(ColorMask::RED | ColorMask::BLUE);

    dbg!(point2(0, 2).is_inside(point2(2, 3)));


    let size = point2(2, 3);

    let grid = Grid2::from_fn(size, |p| p.x + 10 * p.y);
    
    for y in (0..size.y).rev()
    {
        for x in 0..size.x
        {
            let p = point2(x, y);
            dbg!(p);
            dbg!(grid.is_index_invalid(p));
            print!("{:2} ", grid[p]);
        }
        println!()
    }
        

    let mut indice = 0;
    assert_eq!(grid[indice], grid[point2(0,0)]);
    assert_eq!(grid[indice], 0);

    indice += 1;
    assert_eq!(grid[indice], grid[point2(1,0)]);
    assert_eq!(grid[indice], 1);

    indice += 1;
    assert_eq!(grid[indice], grid[point2(0,1)]);
    assert_eq!(grid[indice], 10);

    indice += 1;
    assert_eq!(grid[indice], grid[point2(1,1)]);
    assert_eq!(grid[indice], 11);

    indice += 1;
    assert_eq!(grid[indice], grid[point2(0,2)]);
    assert_eq!(grid[indice], 20);

    indice += 1;
    assert_eq!(grid[indice], grid[point2(1,2)]);
    assert_eq!(grid[indice], 21);
}