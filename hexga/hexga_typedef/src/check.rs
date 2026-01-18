//! compile time to check if the feature are correctly enable 

// Ensure that exactly one float flag is enabled
#[cfg(not(any(
    feature = "float_are_32_bits",
    feature = "float_are_64_bits",
    feature = "float_are_size_bits"
)))]
compile_error!(
    "Missing one of the following features: \
     `float_are_32_bits`, `float_are_64_bits`, or `float_are_size_bits`."
);

#[cfg(any(
    all(feature = "float_are_32_bits", feature = "float_are_64_bits"),
    all(feature = "float_are_32_bits", feature = "float_are_size_bits"),
    all(feature = "float_are_64_bits", feature = "float_are_size_bits"),
    all(feature = "float_are_32_bits", feature = "float_are_64_bits", feature = "float_are_size_bits")
))]
compile_error!(
    "Multiple float flags enabled. Please enable only one of: \
     `float_are_32_bits`, `float_are_64_bits`, or `float_are_size_bits`."
);


// None enabled
#[cfg(not(any(
    feature = "int_are_8_bits",
    feature = "int_are_16_bits",
    feature = "int_are_32_bits",
    feature = "int_are_64_bits",
    feature = "int_are_size_bits"
)))]
compile_error!(
    "Missing one of the following features: \
     `int_are_8_bits`, `int_are_16_bits`, `int_are_32_bits`, \
     `int_are_64_bits`, or `int_are_size_bits`."
);

// More than one enabled
#[cfg(any(
    all(feature = "int_are_8_bits", feature = "int_are_16_bits"),
    all(feature = "int_are_8_bits", feature = "int_are_32_bits"),
    all(feature = "int_are_8_bits", feature = "int_are_64_bits"),
    all(feature = "int_are_8_bits", feature = "int_are_size_bits"),
    all(feature = "int_are_16_bits", feature = "int_are_32_bits"),
    all(feature = "int_are_16_bits", feature = "int_are_64_bits"),
    all(feature = "int_are_16_bits", feature = "int_are_size_bits"),
    all(feature = "int_are_32_bits", feature = "int_are_64_bits"),
    all(feature = "int_are_32_bits", feature = "int_are_size_bits"),
    all(feature = "int_are_64_bits", feature = "int_are_size_bits"),
))]
compile_error!(
    "Multiple int size features enabled. Please enable only one of: \
     `int_are_8_bits`, `int_are_16_bits`, `int_are_32_bits`, \
     `int_are_64_bits`, or `int_are_size_bits`."
);