# Composition example demonstrating integer overflow arithmetic

This repository contains an overflow arithmetic library component (in the `library` directory) that exports an implementation of `overflowing-add` which returns the resulting value from addition plus a boolean indicating whether an arithmetic overflow would occur. 

Additionally a Spin HTTP application (in the `example` directory) imports the library to compute the overflowing addition of 2 values that are passed to the handler within each requests body as a json value (e.g. `{"x": 42, "y": 2}`). On success, the application returns a json encoded response reporting the value of addition and whether overflow occured, e.g. `{"value": 44, "overflow": false}`. If the returned `"overflow"` field is `true` (indicating overflow has occurred), the `"value"` field contains the wrapped value.

The example Spin application is composed with the overflow arithmetic library to create the application's component wasm which is then served by the `spin-cli`.

## Prerequisites
`cargo-component`
`spin v2.3.0`
`wasm-tools`

## Running the example
```
$ spin up --build -f example
```

In a separate window:
```
curl -d '{"x": 100, "y": 200}' localhost:3000/add
{"overflow":false,"value":300}

# NOTE: the value of "x" in the following example is i32::MAX
curl -d '{"x": 2147483647, "y": 1}' localhost:3000/add
{"overflow":true,"value":-2147483648}
```