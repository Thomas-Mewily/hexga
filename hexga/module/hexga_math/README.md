🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.  
It is subject to **breaking changes** in future releases.  
Use it at your own risk, and keep in mind that the API may change in future versions.

## HexGa Math

Provide math related structure for multiple dimension, with a lot of typedef to them like :

- Vector (`Vec2`,`Vec3`,`Vec4` for a vector of float,... `Point2`,`Point3`,`Point4` for int...)

- Rectangle (`Rect2`,`Rect3`,`Rect4` for float, `Rect2P`,`Rect3P`,`Rect4P` for int/point)

- Matrix (`Matrix`, `Mat2`,`Mat3`,`Mat4` for float, `Mat2P`,`Mat3P`,`Mat4P` for int , `SquareMatrix`...) 

- Grid (`Grid2`, `Grid3`, `Grid4`...)

Also provide 

- some lightweight unit of measurement : `Time`, `Angle`.
`assert_eq!([1,2].degree(), [1.degree(),2.degree()])`

- some prefix : kilo, giga, mega...
`assert_eq!(1.kilo(), 1000)`, 
        
- Color (ColorRGBA, ColorHSLA). *<- They will probably be moved into another crate for graphics, but for now they stay here (because of macro)*

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.