## Io Abstraction

- hexga_io:
    - Specific crates for the IO (read/write a file. Iter into a folder. Check if a path exist.)
    - Be able to cache a file into raw memory (can be used for WASM where no file system exist. Use a proc macro to load in cache all file in a folder to have the FS in Ram)


- hexga_encoding:
    - expose the supported file extension.
    - Single file extension


- hexga_asset
    - factorize a Path <=> Value.
    - Asset<T>
    - Can be hot loaded.
    - Can be Auto Saved if needed (saved every X dirty/DerefMut, saved every X minutes)