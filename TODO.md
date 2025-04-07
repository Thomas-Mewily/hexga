HexGa : Highly Extensible (GAme | General Application) Lib Our

- Add Grid

- GenVec : 
    - null qui se sérialize en T::MAX pas bon en cas de changement du type de la gen (faire une enum)
    - Serialization des null
    - fonction pour reset les id


- Name2Reserve : réserver plus de nom sous crate io

- GenHashMap

- transition using a empty struct : `struct Transition;`


- Operateur binaire/assignation : aussi les impl pour des références dont Self est copiable pour les
    - matrix
    - angle
    - time

- angle & time : struct X<T>(T) + faire macro pour wrap op binaires et autres

- RectangleTree<Pri,const N>
- Word<T,Pri,N> : monde multi agent où chaque agent a peut être un rectangle englobant (utilise RectangleTree)

- GridSlice/GridPortion