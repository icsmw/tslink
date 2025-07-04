[![LICENSE](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE.txt)
[![](https://github.com/icsmw/tslink/actions/workflows/on_pull_request.yml/badge.svg)](https://github.com/icsmw/tslink/actions/workflows/on_pull_request.yml)
![Crates.io](https://img.shields.io/crates/v/tslink)

# tslink

`tslink` represents Rust types as `TypeScript` types. It helps to create the npm package (based on node module) with all necessary definitions and types.

# Table of Contents

1. [How it can be useful](#how-it-can-be-useful)
2. [Building](#building)
3. [Output](#output)
4. [Structs](#structs)
5. [Enums](#enums)
6. [Usage](#usage)

-   [Attributes](#attributes)
-   [Multiple attributes](#multiple-attributes)
-   [Struct to TypeScript class](#struct-to-typescript-class)
-   [Struct/Enum to TypeScript interface](#structenum-to-typescript-interface)
-   [Async methods/functions](#async-methods/functions)
-   [Callbacks in methods/functions](#callbacks-in-methodsfunctions)
-   [Naming methods/fields](#naming-methodsfields)
-   [Binding data. Arguments binding.](#binding-data.-arguments-binding.)
-   [Binding data. Result/Errors binding.](#binding-data.-resulterrors-binding.)
-   [Exception suppression](#exception-suppression)
-   [Usage with node-bindgen](#usage-with-node-bindgen)


7. [Import only](#import-only)
8. [Configuration](#configuration)
9. [QA and Troubleshooting](#qa-and-troubleshooting)

## How it can be useful?

### Node modules

If you are developing a node module based on Rust for example using `node-bindgen` crate, `tslink` will generate an npm package with all necessary TypeScript definitions. It helps much with the integration of a node module and testing.

### Sharing types

If you are developing for example a server part on Rust and have a client part on TypeScript, you might be interested in sharing some types from Rust into TypeScript world. Requests or responses can be represented as TypeScript definitions in `*.ts` files.

## Building

Because tslink produces artifacts, by default any IO operations from tslink side would be skipped. This is because compilation can be triggered by multiple reasons (clippy, rust analyzer, etc) and it gives unpredictable IO operations in the scope of the same files and IO errors as a result.
To allow tslink to produce artifacts environment variable `TSLINK_BUILD` should be used with any positive value (`true`, `1`, `on`).

```ignore
export TSLINK_BUILD=true && cargo build
```

> ☞ **NOTE**: tslink only creates a representation of the future node module in JavaScript and TypeScript. To create a native node module a crate `node-bindgen` can be used.

## Output

Based on Rust code `tslink` generates:

-   javascript (`*.js`) for the npm package (library)
-   type definitions file (`*.d.ts`)
-   optionally TypeScript file (`*.ts`) with interfaces

For example for an npm package `tslink` generates:

```ignore
- destination_path
    - lib.d.ts     # TypeScript definition
    - lib.js       # Javascript module representation
    - package.json # NPM package description
```

Optionally `tslink` can generate `*.ts` files. Such files aren't a part of an npm-package and are used just to "share" types between Rust and TypeScript. As soon as `*.ts` files aren't part of an npm-package a destination path for it should be defined separately.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;
#[tslink(target = "./target/selftests/interfaces/interfaces.ts")]
struct TestingA {
    pub p8: u8,
    pub p16: u16,
    pub p32: u32,
    pub p64: u64,
    pub a64: u64,
}
```

Will generate `./target/selftests/interfaces/interfaces.ts` with:

```ignore
export interface TestingA {
    p8: number;
    p16: number;
    p32: number;
    p64: number;
    a64: number;
}
```

> **Note**. Actually type `u64`, `i64`, `usize` and `isize` should be represented as `BigInt`, but in most cases `number` is used instead. To make your code safe, it's better to represent it as `BigInt`. To do it, you can use the setting `int_over_32_as_big_int` in `Cargo.toml` file (section `[package.metadata.tslink]`) with the value `true` (default - `false`).

## Structs

`tslint` represents struct by default as an interface, but it also can be represented as `class`. Class representation should be used in case if struct has some methods and methods are propagated into the node module.

If a struct is used as a type definition, better to use an interface representation.

For next Rust code `tslink` generates `*.js` and `*.d.ts` files.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;
# use std::collections::HashMap;
#[tslink(class)]
struct StructureA {
    pub p8: u8,
    pub p16: u16,
    pub p32: u32,
    pub p64: u64,
    pub a: (u32, u64),
    pub b: Vec<u64>,
    pub c: HashMap<String, u64>,
    pub s: String,
    pub k: Option<String>,
}

#[tslink]
impl StructureA {
    #[tslink]
    pub fn method_a(&self, abs: u8) -> u8 {
        0
    }
}
```

Typescript type definition (`*.d.ts`) representation

```ignore
export declare class StructureA {
    p8: number;
    p16: number;
    p32: number;
    p64: number;
    a: [number, number];
    b: number[];
    c: { [key: string]: number };
    s: string;
    k: string | null;
    method_a(abs: number): number;
}
```

## Enums

### Flat Enum Representation

A flat enum will be represented as a classic TypeScript `enum`.

```rust
# #[macro_use] extern crate tslink;
# use tslink::tslink;
#[tslink]
enum FlatEnum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Nine,
}
```

This Rust enum will be converted to the following TypeScript definition in `*.d.ts`:

```typescript
export enum FlatEnum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Nine,
}
```

### Nested Enum Representation

Any Rust enum with variants that include nested types will, by default, be represented as an `interface` in TypeScript. Here is an example:

```rust
# #[macro_use] extern crate tslink;
# use tslink::tslink;
#[tslink]
enum SomeEnum {
    One,
    Two,
    Three(u8),
    Four(u8, u16, u32),
    Five((String, String)),
    Six(Vec<u8>),
}
```

This will be converted to the following TypeScript definition in `*.d.ts`:

```typescript
export interface SomeEnum {
    One?: null;
    Two?: null;
    Three?: number;
    Four?: [number, number, number];
    Five?: [string, string];
    Six?: number[];
}
```

### Enum Representation Configuration

You can customize how enums are represented in TypeScript by using the `enum_representation` setting in your `Cargo.toml` file. The following options are available:

#### 1. `flat` (default)

This option represents the enum as a TypeScript `interface` with optional fields corresponding to each variant of the Rust enum. Example:

```typescript
export interface SomeEnum {
    One?: null;
    Two?: null;
    Three?: number;
    Four?: [number, number, number];
    Five?: [string, string];
    Six?: number[];
}
```

#### 2. `union`

This option represents the enum as a TypeScript `type` that uses a union of objects. Each variant is represented as an object with a single field named after the variant. Example:

```typescript
export type SomeEnum =
    { One: null; } |
    { Two: null; } |
    { Three: number; } |
    { Four: [number, number, number]; } |
    { Five: [string, string]; } |
    { Six: number[]; };
```

#### 3. `discriminated`

This option is similar to `union`, but variants without associated data are represented as string literals. Example:

```typescript
export type SomeEnum =
    "One" |
    "Two" |
    "Three" |
    { Four: [number, number, number]; } |
    { Five: [string, string]; } |
    { Six: number[]; };
```

### Why Multiple Representations?

The flexibility in enum representation is necessary because different serialization and deserialization methods between Rust and TypeScript may require specific formats. This allows developers to choose the representation that best fits their use case.

 

## Usage

### Attributes

| Attribute                       | Usage                                              | Description                                                                                                                                                                            | Applied To                   |
| ------------------------------- | -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| `class`                         | `#[tslink(class)]`                                 | Tells tslink create TypeScript class instead `interface`                                                                                                                               | struct                       |
| `ignore`                        | `#[tslink(ignore)]`                                | Ignore current struct's field or method                                                                                                                                                | struct method                |
| `ignore = "list"`               | `#[tslink(ignore = "field_a; field_b; method_a")]` | List of fields/methods, which should be ignored. Can be defined only on struct declaration.                                                                                            | struct                       |
| `snake_case_naming`             | `#[tslink(snake_case_naming)]`                     | Renames struct's field or method into snake case naming (`my_field_a` became `myFieldA`)                                                                                               | struct method, functions     |
| `rename = "name"`               | `#[tslink(rename = "newNameOfFieldOrMethod")]`     | Renames struct's methods or functions into given name                                                                                                                                  | struct method and functions  |
| `constructor`                   | `#[tslink(constructor)]`                           | Marks current methods as constructor. Indeed can be defined only for method, which returns `Self`.                                                                                     | struct method returns `Self` |
| `target = "path"`               | `#[tslink(target = "./path_to/file.ts")]`          | Tells tslink save TypeScript definitions `*.ts` into given file                                                                                                                        | struct, enum                 |
| `module = "mod_name"`           | `#[tslink(target = "./path_to/file.ts", module = "mod_name")]` | Link `struct`/`enum` to target module. Uses with `target` only.                                                                                                            | struct, enum                 |
| `exception_suppression`         | `#[tslink(exception_suppression)]`                 | By default in case of error method/function throws a JavaScript exception. If "exception_suppression" is used, method/function returns an JavaScript Error instead throwing exceptions | struct methods, functions    |
| `result = "json"`               | `#[tslink(result = "json")]`                       | Converts `Ok` case in `Result<T, _>` into JSON                                                                                                                                         | struct methods, functions    |
| `error = "json"`                | `#[tslink(error = "json")]`                        | Converts `Err` case in `Result<_, E>` into JSON                                                                                                                                        | struct methods, functions    |
| `fn_arg_name = "ref_to_entity"` | `#[tslink(data = "MyStruct")]`                     | Binds argument type with struct/type/enum on Rust side                                                                                                                                 | struct methods, functions    |

### Multiple attributes

Multiple attributes can be defined

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;
#[tslink(
    class,
    target = "./target/selftests/interfaces/interfaces.ts; ./target/selftests/interfaces/interfaces.d.ts",
    ignore = "_p8;_p16;_p32"
)]
struct MyStruct {
    pub _p8: u8,
    pub _p16: u16,
    pub _p32: u32,
    pub _p64: u64,
    pub a64: u64,
}

impl MyStruct {
    #[tslink(snake_case_naming, exception_suppression)]
    fn my_method(&self) -> Result<i32, String> {
        Err("test".to_string())
    }
}

#[tslink(snake_case_naming, exception_suppression)]
fn my_function() -> Result<i32, String> {
    Err("test".to_string())
}
```

### Struct to TypeScript class

To reflect `struct` into TypeScript class `#[tslink(class)]` should be used, because by default tslink represents `struct` as `interface`.

If struct has specific constructor, such method should be marked with `#[tslink(constructor)]`.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;
#[tslink(class)]
struct MyStruct {
    pub field_a: u8,
}

impl MyStruct {
    #[tslink(constructor)]
    fn new() -> Self {
        Self { field_a: 0 }
    }
    #[tslink]
    fn my_method(&self) -> Result<i32, String> {
        Err("test".to_string())
    }
}
```

If `struct` doesn't have fields #[tslink(class)] can be applied to `impl` directly.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;

struct MyStruct { }

#[tslink(class)]
impl MyStruct {
    #[tslink(constructor)]
    fn new() -> Self {
        Self { }
    }
}
```

> ☞ **NOTE**: if your structure has constructor mark this method with `#[tslink(constructor)]` is obligatory to allow tslink represent construtor in JS reflection.

### Struct/Enum to TypeScript interface

To reflect `struct` or `enum` into TypeScript `interface` `#[tslink]` should be used.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;
#[tslink]
struct MyStruct {
    pub field_a: u8,
    pub field_b: u8,
    pub field_c: u8,
}

#[tslink]
enum MyFlatEnum {
    One,
    Two,
    Three,
}

#[tslink]
enum MyEnum {
    One(String),
    Two(i32, i32),
    Three,
}
```

Note, "flat" enum (`MyFlatEnum`) will be converted into classic TypeScript `enum`, but composite enum (`MyEnum`) will converted into `interface`.

### Async methods/functions

Result of async methods/function will be represented as `Promise` on TypeScript side.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;
struct MyStruct {
}

#[tslink(class)]
impl MyStruct {

    #[tslink]
    async fn my_async_method(&self) -> i32 {
        0
    }
}
```

Would be represented as

```ignore
export declare class MyStruct {
    my_async_method(): Promise<number>;
}
```

> ☞ **NOTE**: suppression JS exceptions doesn't make sense with promises and using this attribute will not affect any.

### Callbacks in methods/functions

The recommended way to define callback is using generic types.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;
struct MyStruct {}

#[tslink(class)]
impl MyStruct {
    #[tslink]
    fn test_a<F: Fn(i32, i32)>(&self, callback: F) {
        callback(1, 2);
    }
}
```

Would be represented as

```ignore
export declare class MyStruct {
    testA(callback: (arg0: number, arg1: number) => void): void;
}
```

### Constants

`tslink` supports an export of constants

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;

#[tslink]
const MY_CONST: &str = "Hello World!";
```

Would be represented as

```ignore
export const MY_CONST = "Hello World!";
```

**Note**. Array constants aren't supported on TypeScript layer.

### Naming methods/fields

TypeScript/JavaScript standard of naming: snake case naming. Some crates like `node-bindgen` automatically rename fields and methods based on this rule. To fit this behavior `tslink` should know, which fields/methods should be renamed.

The easiest way would be using `#[tslink(snake_case_naming)]` on a level of method/field. Or at some very specific use-cases can be used `#[tslink(rename = "newNameOfFieldOrMethod")]` to give method/field some specific name.

```
# #[macro_use] extern crate tslink;
# use tslink::tslink;

#[tslink(class, snake_case_naming)]
struct MyStruct {
    field_a: i32,
}

#[tslink(class)]
impl MyStruct {
    #[tslink(snake_case_naming)]
    fn my_method_a(&self) -> i32 {
        0
    }
    #[tslink(rename = "newNameOfMethod")]
    fn my_method_b(&self) -> i32 {
        0
    }
}
```

Would be represented as

```ignore
export declare class MyStruct {
    thisIsFieldA: number;
    myMethodA(): number;
    newNameOfMethod(): number;
}
```

> ☞ **NOTE**: `#[tslink(rename = "CustomName")]` cannot be used for renaming fields, but `snake_case_naming` can be applied to fields on a top of struct.

### Binding data. Arguments binding.

Methods/function arguments types can be bound with some data types on level on Rust with `#[tslink(data = "MyStruct")]`.

```
#[macro_use] extern crate tslink;
use serde::{Deserialize, Serialize};
use tslink::tslink;

// Define error type for bound method
#[tslink]
#[derive(Serialize, Deserialize)]
struct MyError {
    msg: String,
    code: usize,
}

// Make possible convert serde_json error into our error implementation
impl From<serde_json::Error> for MyError {
    fn from(value: serde_json::Error) -> Self {
        MyError {
            msg: value.to_string(),
            code: 1,
        }
    }
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct MyData {
    pub a: i32,
    pub b: i32,
}

struct MyStruct { }

#[tslink(class)]
impl MyStruct {
    #[tslink(
        my_data = "MyData",
        error = "json",
    )]
    fn get_data(&self, my_data: String) -> Result<i32, MyError> {
        println!("my_data.a = {}", my_data.a);
        println!("my_data.b = {}", my_data.b);
        Ok(my_data.a + my_data.b)
    }}
```

Will be represented as

```ignore
export declare class MyStruct {
    getData(my_data: MyData): number;
}
```

**Important**

1. tslink converts bound data into `JSON string`. It requires `serde`, `serde_json` as dependencies in your project.
2. Because parsing of `JSON string` potentially can be done with errors, the method/function should return only `Result<T, E>`
3. Because `serde_json` returns `serde_json::Error` error type of result should be convertable from `serde_json::Error`.
4. In most cases you would use binding of data with `#[tslink(error = "json")]` because it allows you to use your implementation of error. And it's a recommended way.
5. In the declaration of the method/function on Rust side, the type of argument should be `String` (ex: `fn get_data(&self, my_data: String) -> Result<MyData, MyError>`), but in the body of your method/function this argument will be considered as bounded type.
6. And bound type and error should implement `Serialize` and `Deserialize`

### Binding data. Result/Errors binding.

To bind error with some of your custom types `#[tslink(error = "json")]` should be used, like it was shown in "Binding data. Arguments binding.". Like an argument error will be serialized into `JSON string` on Rust level and parsed from `JSON string` on TypeScript/JavaScript level.

To bind result with some of your custom data type `#[tslink(result = "json")]` should be used.

```
#[macro_use] extern crate tslink;
use serde::{Deserialize, Serialize};
use tslink::tslink;

// Define error type for bound method
#[tslink]
#[derive(Serialize, Deserialize)]
struct MyError {
    msg: String,
    code: usize,
}

// Make possible convert serde_json error into our error implementation
impl From<serde_json::Error> for MyError {
    fn from(value: serde_json::Error) -> Self {
        MyError {
            msg: value.to_string(),
            code: 1,
        }
    }
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct MyData {
    pub a: i32,
    pub b: i32,
    pub c: String,
}

struct MyStruct { }

#[tslink(class)]
impl MyStruct {
    #[tslink(
        my_data = "MyData",
        result = "json",
        error = "json",
    )]
    fn get_data(&self, my_data: String) -> Result<MyData, MyError> {
        Ok(MyData {
            a: my_data.a + 1,
            b: my_data.b + 1,
            c: format!("{}{}", my_data.c, my_data.c),
        })
    }}
```

Will be represented as

```ignore
export declare class MyStruct {
    getData(my_data: MyData): MyData;
}
```

**Important**

1. tslink converts bound data into `JSON string`. It requires `serde`, `serde_json` as dependencies in your project.
2. Because parsing of `JSON string` potentially can be done with errors, the method/function should return only `Result<T, E>`
3. Because `serde_json` returns `serde_json::Error` error type of result should be convertable from `serde_json::Error`.
4. In most cases you would use binding of data with `#[tslink(error = "json")]` because it allows you to use your implementation of error. And it's a recommended way.
5. In the declaration of the method/function on Rust side, the type of argument should be `String` (ex: `fn get_data(&self, my_data: String) -> Result<MyData, MyError>`), but in the body of your method/function this argument will be considered as bounded type.
6. And result type and error should implement `Serialize` and `Deserialize`.

### Exception suppression

Would be exception thrown or no is up to the library/crate, which is used to create a node module. For example `node-bindgen` throws exceptions on JavaScript level as soon as a method/function is done with an error. But tslink allows customizing this scenario.

By default exception suppression is off and any error on Rust level became an exception on JavaScript level.

Let's take a look to the previous example:

```
# #[macro_use] extern crate tslink;
# use serde::{Deserialize, Serialize};
# use tslink::tslink;
# #[tslink]
# #[derive(Serialize, Deserialize)]
# struct MyError {
#     msg: String,
#     code: usize,
# }
# // Make possible convert serde_json error into our error implementation
# impl From<serde_json::Error> for MyError {
#     fn from(value: serde_json::Error) -> Self {
#         MyError {
#             msg: value.to_string(),
#             code: 1,
#         }
#     }
# }
struct MyStruct { }

#[tslink(class)]
impl MyStruct {
    #[tslink(
        error = "json",
    )]
    fn get_data(&self, my_data: String) -> Result<i32, MyError> {
        Err(MyError { msg: "Test".to_string(), code: 1})
    }}
```

Will be represented as

```ignore
export declare class MyStruct {
    getData(my_data: MyData): number;
}
```

Method `getData` returns `MyData` but in case of error JavaScript exception will be thrown.

Using `#[tslink(exception_suppression)]` we can change it.

```
# #[macro_use] extern crate tslink;
# use serde::{Deserialize, Serialize};
# use tslink::tslink;
# #[tslink]
# #[derive(Serialize, Deserialize)]
# struct MyError {
#     msg: String,
#     code: usize,
# }
# // Make possible convert serde_json error into our error implementation
# impl From<serde_json::Error> for MyError {
#     fn from(value: serde_json::Error) -> Self {
#         MyError {
#             msg: value.to_string(),
#             code: 1,
#         }
#     }
# }
struct MyStruct { }

#[tslink(class)]
impl MyStruct {
    #[tslink(
        error = "json",
        exception_suppression
    )]
    fn get_data(&self, my_data: String) -> Result<i32, MyError> {
        Err(MyError { msg: "Test".to_string(), code: 1})
    }}
```

Will be represented as

```ignore
export declare class MyStruct {
    getData(my_data: MyData): number | (Error & { err?: MyError});
}
```

Now `getData` returns or `number`, or `Error & { err?: MyError}` in case of error, but an exception is suppressed.

> Use or not to use this feature is up to the developer, but in general it's a good way to reduce `try/catch` blocks on JavaScript/TypeScript side and be ready for errors in places where it's potentially possible.

### Usage with node-bindgen

`node-bindgen` crate allows to create native node module and with tslink to get a complete npm project.

There just one rule to common usage - call of `#[tslink]` should be always above of call `#[node_bindgen]`

```ignore
#[macro_use] extern crate tslink;
use tslink::tslink;
use node_bindgen::derive::node_bindgen;

struct MyScruct {}

#[tslink(class)]
#[node_bindgen]
impl MyScruct {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn inc_my_number(&self, a: i32) -> i32 {
        a + 1
    }
}
```

Please **note**, `node-bindgen` by default applies snake case naming to methods. You should use `#[tslink(snake_case_naming)]` to consider this moment.

By default `node-bindgen` creates `index.node` in `./dist` folder of your `root`. In `Cargo.toml` file should be defined suitable path in section `[package.metadata.tslink]`:

File: `./Cargo.toml` (in a `root` of project):

```ignore
[project]
...

[lib]
...

[package.metadata.tslink]
node = "./dist/index.node"

[dependencies]
...
```

Full example of `node-bindgen` usage is [here](https://github.com/icsmw/tslink/tree/master/examples/node_bindgen). To start it:

```sh
git clone https://github.com/icsmw/tslink.git
cd tslink/examples/node_bindgen
sh ./run_test.sh
```

## Import only

`tslink` can also be used to import Rust types into TypeScript. For this, you can use the `target` directive to specify the file name where the `*.ts` files will be saved. Additionally, to ensure modularity, you can use the `module` directive along with the `target` directive to bind a specific data type to a specific module.

For example let's take a look on next rust code

```ignore
// ./lib.rs

mod module_a;
mod module_b;

pub use module_a::*;
pub use module_b::*;

// ./module_a.rs
use tslink::tslink;

#[tslink(target = "./output/module_a.ts", module = "module_a")]
pub enum FieldA {
    One,
    Two,
    Three,
}

#[tslink(target = "./output/module_a.ts", module = "module_a")]
pub enum FieldB {
    One(String),
    Two((u32, u32)),
    Three(FieldA),
}

#[tslink(target = "./output/module_a.ts", module = "module_a")]
pub struct StructA {
    pub a: FieldA,
    pub b: FieldB,
}

// ./module_b.ru
use crate::{FieldA, FieldB, StructA};
use tslink::tslink;

#[tslink(target = "./output/module_b.ts", module = "module_b")]
pub enum EntityA {
    One,
    Two,
    Three,
}

#[tslink(target = "./output/module_b.ts", module = "module_b")]
pub enum EntityB {
    One(String),
    Two((u32, u32)),
    Three(EntityA),
}

#[tslink(target = "./output/module_b.ts", module = "module_b")]
pub struct OtherStruct {
    pub a: EntityA,
    pub b: EntityB,
    pub c: StructA,
    pub d: FieldA,
    pub e: FieldB,
}
```

Based on this `tslink` will generate next files
```ignore
- ./output
    - index.ts      - index with all types
    - module_a.ts   - types related to module_a
    - module_b.ts   - types related to module_b
```

For example `module_b` will look like

```typescript
export interface EntityB {
    One?: string;
    Two?: [number, number];
    Three?: EntityA;
}
export enum EntityA {
    One,
    Two,
    Three,
}
import { FieldB } from "./module_a";
import { StructA } from "./module_a";
import { FieldA } from "./module_a";
export interface OtherStruct {
    a: EntityA;
    b: EntityB;
    c: StructA;
    d: FieldA;
    e: FieldB;
}
```

## Configuration

Global configuration of `tslink` can defined in section `[package.metadata.tslink]` of `Cargo.toml` file in the root of your project. It's required in most cases. This settings allows to define a path to a native node module, which will be bound with an npm package.

But if tslink is used only to generate interfaces in `*.ts` files, a configuration file can be skipped.

Example of `./Cargo.toml` with `tslink` settings:

```ignore
[package]
name = "tslink-test"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]
path = "rs/lib.rs"

[package.metadata.tslink]
# [required] path to native node module
node = "./dist/index.node"

# [optional] global rule of renaming (can be: "method" or "fields" or both - "methods,fields")
snake_case_naming = "methods"

# [optional] global rule for javascript exception suppression
exception_suppression = true

# [optional] in true will use <BigInt> (instead <number>) for <u64>, <i64>, <usize> and <isize> (default - false)
int_over_32_as_big_int = true
```

| Field                                 | Required | Values                                        | Description                                      |
| ------------------------------------- | -------- | --------------------------------------------- | ------------------------------------------------ |
| `node = "path_to_native_node_module"` | yes      | path to file                                  | path to native node module                       |
| `snake_case_naming = "rule"`          |          | "`methods`", "`fields`" or "`methods,fields`" | global rule of renaming                          |
| `exception_suppression = true`        |          | `bool`                                        | global rule for javascript exception suppression |
| `int_over_32_as_big_int = true` | | `bool` | using of BigInt type |

**Note**: Prior to version `0.4.1`, the `[tslink]` section at the root level of `Cargo.toml` was used. Starting from version `0.4.2`, the default location for configuration has been moved to `[package.metadata.tslink]` to ensure full compatibility with Cargo's schema. However, all versions from `0.4.2` onward retain backward compatibility and still check the deprecated root-level `[tslink]` section. In such cases, Cargo will emit a warning about unexpected keys in the manifest.

## QA and Troubleshooting

> **Q**: tslink doesn't create any files
>
> **A**: make sure, the environment variable `TSLINK_BUILD` has been exported with `true` or `1`

---

> **Q**: rust-analyzer reports IO errors from tslink
>
> **A**: remove the environment variable `TSLINK_BUILD` or set it into `false` or `0`

---

> **Q**: what is it `./target/selftests`?
>
> **A**: these are artifacts, which tslink created with `cargo test`. It's safe to remove.

---

> **Q**: Does tslink create native node module (like `index.node`)
>
> **Q**: No, tslink only creates a representation of the future node module in JavaScript and TypeScript. To create a native node module a crate `node-bindgen` can be used.

---

> **Q**: With `node-bindgen` I get errors on JavaScript side like "no method_call_b() on undefined".
>
> **Q**: Note, `node-bindgen` by default applies snake case naming to methods. You should use `#[tslink(snake_case_naming)]` to consider this moment.
