# The `.mf` extension

`.mf` stand for `Multi File`.
When using this format, the underlying markup language (`json`, `ron`...) will be using multiple file.

The extension should be added at the end of the regular extension:

`foo.json` => single json file
`foo.json.mf` => multi file

# Keyword

module: `mod`;
identifier : (['a'-'z']|['A'-'Z']|'_'|' ')*;


# What would be multi file




## SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeStructVariant, and Serializer for primitive (iX, uX, fX, char, bool...)

Serialize them in the markup language.
There is no multi file support.

## SerializeMap and SerializeStruct

Structure field and map entry (where the map Key is a `String` or a `char`) can be saved into multiple file, or in a regular markup language, or even both.

They will be refered as (key, value):
- SerializeStruct => (field_name, field_value)
- SerializeMap => (key, value)

If the key is a valid identifier, then the entry *can* be stored in a file, the value must be serialized using the markup serializer into a subfile where the subfile name correspond to the key.

The remaning unsaved entrie will be saved in a regular SerializeMap/SerializeStruct in an special subfile named `mod.<Markup Extension>`

Example for storing in json:

- a map `{"one":1, "two":2, "three":3}`
or
- a structure `{one:1, two:2, three:3}`

File Represenation:

```
- myMap
  |- one.json      // 1
  |- two.json      // 2
  |- three.json    // 3
```

```
- myMap
  |- mod.json      // {"thee": 3}
  |- one.json      // 1
  |- two.json      // 2
```










Every path can be a file and a directory at the same time.
For that, a special file name mod.X is reserved
