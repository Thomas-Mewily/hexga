HexGa : Highly Extensible (GAme | General Application) Lib Our

## Todo

- Name2Reserve : réserver plus de nom sous crate io

- Io System / Cache / Mediator

- GenVec : 
    - fonction pour reset les id
    - fonction rollback_insert rollback_push rollback_remove_idx ... (pour rollback dans un jeux)

- Grid : divide IGrid in IGridVector (vector/slice impl) and IGrid. Maybe merge Grid and GridParam?


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

- angle & time : struct X<T>(T) + faire macro pour wrap op binaires et autres

- S'intérésser aux structure de quadtree déjà existante
- RectangleTree<Pri,const N>
- Word<T,Pri,N> : monde multi agent où chaque agent a peut être un rectangle englobant (utilise RectangleTree)


## Working On

## Done

- Grid : GridSlice/GridPortion + version mut
~~Grid Deref+Derefmut sur ses slices~~ => Pas possible, impossible d'exprimer les lifetime
+ impl Iterator sur les grid slice

- Graphic : Image : How to serialize it => Custom type + impl IGridParam (still a lot of code)
- Mettre hexga_map_to dans sa propre crate

## Other

- GenHashMap ?
