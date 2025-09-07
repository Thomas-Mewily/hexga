`#stabilized` in the code = need to refactor later when some feature will be stabilized
`#proper_error` in the code = need to define a proper error type for the result instead of ()

## To-do

- move le trait Abs et l'implémenter dans Composite
- 
- limiter les fps

- trait Getter<T> et Setter<T> ? (GetPosition, GetRectangle, GetMatrix...)

- crée les crates:
- pour gérer: les undo redo
- les graphes de manière générique (Vector, GenVec, HashSet) en utilisant Get, GetMut...

- HexgaIo : default extension for saving if not specified + guess extension when loading
- Parse extra extension as argument a give them during serialization. Ex: `mygrid.save("mygrid.flat.ron")` => `flat` is an argument ?


- impl io save/load grid, image/gif, asset game engine

- Name2Reserve : réserver plus de nom sous crate io

- Undo Redo : Impl Action pour vecteur, slice, array, hashmap, hashset... struct field.
    - ne pas imposer d'utiliser un vecteur pour command stack / marker => use collection
    - type de retour
    - composable / limite scripting ?

- Use the serde deserialize to parse command by user / auto completion. ex : command written in a video game from a player
ex : minecraft : `/setblock 10 20 30 grass`
contextual action ? : `/setblock ~10 ~5 8 grass` <- relative position to the caller ?
position is a matrix ? for each component x/y/z: `positionFixed positionRelative`

- Io System / Cache / Mediator

- transition fn inside an empty struct to make it extensible (trait pattern) : `struct Transition;`

- Operateur binaire/assignation : aussi les impl pour des références dont Self est copiable pour les
    - matrix
    - angle
    - time...
    Le faire de manière clean :
        - `Op<&T> for T`
        - `Op<T> for &T`
        - `Op<&T> for &T`
        ... Self and T copiable + support base op

Grid from matrice ?

- Math : Ray (rayon)


- Hexga math : easing / tweening fn

- S'intérésser aux structure de quadtree déjà existante
- RectangleTree<Pri,const N>
- Word<T,Pri,N> : monde multi agent où chaque agent a peut être un rectangle englobant (utilise RectangleTree)


- VecX : move_relative to the current vector direction
vec2(1,0) => forward to the current dir
vec2(-1,0) => backward...
projeter orhtogonal ?

- ColorRGB (without A). Support for jpg / jpeg that don't support an A

- ECS ? (not imposed, but the ability the create component, delete them, iterate...)


- SDF Signed Distance Field
https://iquilezles.org/articles/distfunctions/
https://iquilezles.org/articles/distfunctions2d/
https://wbrbr.org/publications/LipschitzPruning/

- SVF : Signed Vector Field (can also be used for collision)

## Working On

## Done

-  Move the Abs trait from Math to Number

- Grid : GridSlice/GridPortion + version mut
~~Grid Deref+Derefmut sur ses slices~~ => Pas possible, impossible d'exprimer les lifetime
+ impl Iterator sur les grid slice

- Graphic : Image : How to serialize it => Custom type + impl IGridParam (still a lot of code)
- Mettre hexga_map_to dans sa propre crate

- GenVec :
    - fonction pour reset les id
    - fonction rollback_insert rollback_push rollback_remove_idx ... (pour rollback dans un jeux)
    - clear() différent de remove_all()
    - drain ?
    - impl Length + Clear

- angle & time : struct X<T>(T) + faire macro pour wrap op binaires et autres

- iterator with sample() and step() for Float and unit of mesure (Time, Angle...)

- Grid : divide IGrid in IGridVector (vector/slice impl) and IGrid. Maybe merge Grid and GridParam?
Impl size_hint() and ExactSizeIterator for view and view mut

trait ToAngle, ToTime : différent trait pour chaque unité (pour pouvoir les importer de manière séparer)
90.s()
45.min()

ToTimeS
ToTimeMin

## Other

- GenHashMap ?
