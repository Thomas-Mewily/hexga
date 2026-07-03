`#stabilized` in the code = need to refactor later when some feature will be stabilized
`#proper_error` in the code = need to define a proper error type for the result instead of ()

## To-do


For io, I want don't want to specify the extension. 

When saving a value a value to a path, if the path don't have any extension, the extension should be deduced from the type itself. 

Ex: 
- `String` support the following extension : ["txt", "md", "csv] ( `load_custom_extensions()` ). The first one "txt" the prefered extension.
- `Image` support ["png", "jpg", "jepg"]

When loading a value from a path, if the path don't have any extension, and if there is only one file with the same name, choose the extension of the file.

trying to load `example` into a folder that contains `example.txt`, the path will be auto corrected to load `example.txt`.
But the folder also contains another file `example.md` then it is ambiguous. The path will not be auto corrected because of the ambiguity and loading `example` will fail since no path match.


File<T>
value : T,
path: T,

Asset<T>
value: T
path: T


Because multiple path can resolve to the same file (`example` and `example.txt`), for data structure like File or Asset (because Asset are factorized, loading the same asset twice for the same file return the same instance), the path need to be canonicalized / normalized.

The path will keep the extension.

When serializing/deserializing

The path in those need to be factorized. It will always use the non extension version ?

``Path


Bijection 1D <=> ND, Grid, Triangle, Spacial (infinite) + View type
Image

Serialisation inside file (support multiple file) different than serde