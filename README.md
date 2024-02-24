# RustToLearn
Rust learning progress with toy examples for review and illustration.

## Syntax

### Hello world:
use 4 spaces as indent

use main as the code entry

#### main
normally return ()

but can return any type that implement the ```std::process::Termination``` trait, which contains a function report that returns an ExitCode, for example: ```Result<(), std::io::Error>```

---

### Hello cargo:
use the following command to create a cargo project with new directory created
```cargo new xxx```

toml file contains the configuration

__package__ section contains the information of this package

__dependencies__ section contains the required packages of code ( known as crates )

use the following command to build the project. the default build is a debug build, therefore executable is in debug folder.
```cargo buld```

use the following to bulid for release, optimization will be used.
```cargo build --release```

lock file is generated in the first build, which keeps track of exact versions of dependencies

use the following command to bulid and run
```cargo run```

use the following comman to check compile without real compile and generating executable
```cargo check```

---

### Variables and mutability
create immutable variable (let is immutable by default).
```let x = 5```

create mutable variable
```let mut x = 5```

constant: 1. cannot use with mut; 2. type must be declared; 3. statically defined (not runtime)
```const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;```

---

### Shadowing
variable are shadowed by scope stack. Inner scope is initialized from its parent scope, but the parent scopee will not be affected by the inner scope.

with shadowing, immutable variable can actually can "mutable" by let expression in the same scope.
```
let x = 5;
let x = x + 1;
```

with shadowing, we can reuse the same name but change the variable type
```
let spaces = "   ";
let spaces = spaces.len();
```

shadowing + let immutable is still different from let mut. shadowing isolates scopes therefore even if variables can be mutable in the current scope, the variable of the same name in the outer scope is not affected. Thus, if we world not like the inner scope affect the outer scope, we should use shadowing + let immutable; if we would like any inner scope change the variable, we should use let mutable.

---

### Data types
Rust is a statically typed language.

By using the syntax, without type explicitly declared. The type is inferred by compiler according to context. 
```
let v = ...;
```
Compiler will cast it to the type to meet the function paramter signature if any.

#### Scalar: 
integer,  float, boolean, chracter

##### integer
type notation: (i/u)(8/16/32/64/128/size(32/64))

type should match with value in type domain and should match regardless of platform or architecture.

overflow cause panic in debug mode, cause 2' complement wrapping in release mode.

handle possible overflow with

 - wrapping_*
 - checked_*
 - overflowing_*
 - saturating_*

for mut variables, overflow is not exposed until runtime

##### float
type notation: f(32/64), f64 is by default

##### bool
type notation: bool 

##### character:
type notation: char

use single quotes to wrap characters; use double quotes to wrap string

the char type is in 4 bytes, representing a Unicode scalar value.

#### Compound:
Two primitive: tuple and array

##### Tuple:
can group objects with various types separated by comma

with fix length

compound / single assign with type inherited

use ```.``` to indexing

##### Array:
group objects with same type.

fixed length

declare type and length

initialized with default object and length

use ```[]``` to indexing

array index overflow result in runtime error (unlike cpp where index overflow some still get result from memory)

---

### Statements and expressions
Statements are instructions that perform some action and do not return a value, like ```let y = 6;```

Expressions evaluate to a resultant value.

---

### Function:
declare function with the following format
```fn func_name(par1: type1, par2: type2) {statements;}```

function with return value. Explicit return using ```return``` statement or implicit return the final expression
```fn func_name(par1: type1, par2: type2) -> return_type {statements; expression}```

---

### Loop:
no condition for block entry, continue or block manually.

use the following syntax to name loop block with label. continue and break can be performed to certain loop block
```'loop_block_name: loop {...}```

---

### While:
Use condition to control the entry of the block
```'while_block_name: while cond {...}```

---

### For:
Iterate data through the container
```for data in container {}``` 

---

### Ownership:
owenership is a way of memory management aside from garbage-collection or explicit allocate and free.

ownership rules:
 - Each value has an owner
 - There can only be one owner at a time
 - When the owner goes out of the scope, the value is dropped. (alike RAII in cpp)

#### Memory allocation / free

Stack: fix size and type, cheap to read and manage

Heap: mutable size, expensize to read and manage

#### Assignment / Function paramter / Function return

heap data: move : the old ownership is invalid (transfer ownership)

stack data (which implement copy traits): copy : the new ownership copy from the old data keeping the ownership valid

#### Reference and Borrowing

Reference is like a pointer. use ```&``` before variable and type. allow refer to value without getting ownership.

Borrrowing: create a reference.

The permission of reference depends on mutability.

Mutable reference can at most have one!! More precisely, the usage span of mutable reference cannot overlap with any other references (both mutable and immutable).

Data race is prevented. Read is not locked, write is locked.

Pointer dangling will not happen. Pointer dangling happens when pointer life span exceeds data life span. 

##### Auto reference / dereference
Rust has auto reference and dereference in some syntax like method invoke. Unlike cpp where . is used for object and -> is used for pointer, rust only has one method operator due to auto reference and dereference


---

### Slicing
use ```..``` to create a range syntax. Head index is 0 by default, tail index is length of the raw data by default.

the slicing type of String is ```&str```, same for string litarals
```let s = "Hello world"```

the slicing type of data collections like array is ```&[T]```

---

### Struct
#### Normal struct
using struct keyword to declare a struct

the field type does not have to be mutable if we would like to modify the struct

read / write the field by ```struct.field```

shorthand is allowed when variable name is the same as the field name

by using ```..struct``` in struct initialization, struct can be partially copied

#### Tuple Struct
only have types, omit field names

can declare a unique type with methods

read / write using integer index

#### Unit Struct
behave similar to ```()```

useful in trait implementation

#### Method
Use ```impl``` keyword over the target struct and declare functions inside the block

The function's first paramter is the reference of the instance ```&self``` (with automatically &struct as type) (which is short for ```self: &Self``` or ```self: &struct```), followed by normal parameter declaration

method can be called through instance
```instance.method()```

or through struct namespace
```struct::method(&instance)``` (associated method)

multiple impl over one namespace is valid as long as no duplicate methods

#### Cmp

impl ```Eq PartialEq Ord PartialOrd``` to support compare (like sort in vector)

---

### Enum
use ```enum``` to declare an Enum.

an enum value can only be one of the Enum variants. Use ```::``` to specify variant

the type of variants can corresponds to the type of struct:
 - no data
 - normal struct (with field name)
 - tuple struct (with sequential types)

Whatever variant type, the variant of an enum is of type of the corresponding Enum.

Move is performed for assignment

#### Enum match
use ```match instance {}``` to specify behavior for every enum variant.

use ```=>``` to bridge variant (binded with __reference of variables__ (struct variable name / temp varaible name)) with scope block ```{}```

all branches mush be covered, if enumerate all branches are unecessary, use ```_``` or ```other```(if values are required) as the default branch

Actually, in the syntax below, sequential pattern match is what actually done. Pattern is like the lambda in python, a pattern is matched if and only if there exist instantiation of variable in pattern such that pattern is equal to instance. That's why pattern ```_```, which contains a variable only, is always matchable. 

```
match instance {
    patter1 => func1,
    pattern2 => func2,
    ...
}
```

#### if let
if let is a pattern match branch syntax
```
if let pattern = instance {}
else {}
```

therefore, match can be considered as the simple syntax for 
```
if let patter1 = instance {}
else if let patter2 = instance {}
...
else {}
```

---

### Manage Projects

#### Crates: 
A tree of modules that produces a library crate or executable (binary) crate.

Binary crates are compiled to be executed, they must have main as the entry function (defined in Cargo.toml)

Library crates do not require a main function and they are not compiled to be executable themselves. The namespace is the package name (defined in Cargo.toml)

Crate roots: entry of the compile to create the root module. The library crate will be the root if exists, main.rs will be the root otherwise.

#### Packages: 
A Cargo feature that lets you build, test, and share crates.

Contain a bundle of crates to form its functionality.

Contain Cargo.toml to describe how to build these crates

Contain as many binary crates as possible but at most one library crate
```
error: failed to parse manifest at `...\project_management\Cargo.toml`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
```
  
A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate. If we cargo build, it works well. But if we cargo run, it requires us to specify which binary crate to run.
```
> ...\project_management> cargo run
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: bin1, project_management
> ...\project_management> cargo build
   Compiling project_management v0.1.0 (...\project_management)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
> ...\project_management>
```

We should specify by using --bin or add default-run = "xxx" to Cargo.toml to keep using cargo run
```
[package]
...version = "0.1.0"
default-run = "[project name]" // if we want to run main.rs
default-run = "[bin name]" // if we want to run bin/[bin name].rs

# customize bin name with target path, override the raw name
[[bin]]
name = "my_bin1"
path = "src/bin/bin1.rs"

[[bin]]
name = "my_bin1"
path = "src/bin/bin1.rs"
```

#### Modules and use: 
Let you control the organization, scope, and privacy of paths.

module is compiled as the following steps
 - start from the crate root: binary crate or library crate
 - declare module / submodule: 

the compiler will then look for 
- crate root inline
- [parent dir]/[module name].rs (new style)
- [parent dir]/[module name]/mod.rs (old style)

new style file path and old one cannot exist as the same time.

for crate root, [parent dir] is src

for submodule, [parent dir] is [parent module dir]

modules can be nested, leading to a module tree
 - path to code in module: as long as the private rules allowed, we can visit the code using ```::```
 - private vs public: code within a module is private to its parent by default. To make it public, using ```pub mod``` instead of ```mod```.
 - ```use``` keyword: create shortcut for path to code

#### Paths: 
A way of naming an item, such as a struct, function, or module. Using ```::``` to make the path.

##### absolute path: 
start from crate root ```crate::xxx::yyy```

keep unchanged as long as the yyy is in the same file

##### relative path: 
start from the current module, using self, super(like ```../``` in the file system)

keep unchanged when the relative position is unchanged (like moved together)

#### Pub for struct and enum
Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. 

Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

#### Use
the ```use``` keyword is to shorthand a path with its final identifier.

make the identifier under crate, i.e. add the path to crate root

the effectiveness of use is restricted by scope, the function scope should be the same as the declaration scope

Idiomatic use path is recommanded, where the function or instance is used with parent identifier kept, so as to avoid misleading to local fucntion or instance (i.e. use namespace to distinguish difference things)

if naming still conflicts, we can ```use ... as ...```

pub use can expose certain identifier under the current module / crate

#### Extenal packages
add package to Cargo.toml ```[dependencies]```

import necessary module / function / instance to scope by ```use``` keyword

nested import
```
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

// --snip--
use std::{cmp::Ordering, io};
// --snip--

==========
use std::io;
use std::io::Write;

use std::io::{self, Write};
```

glob import
```
use std::collections::*;
```

---

### Vector
use ```Vec::new()``` to create new empty vector with ownership

use macro ```vec!([array])``` to create vector with given array with ownership

use ```[]``` to get the reference of the object by index

use ```get()``` to the the option of the reference of the object by index, index overflow is allowed

use ```for o in &(mut) v {}``` to iterate over vector with RO / RW permission.

use ```push(o)``` and ```pop()``` to add or remove object from the vector

use ```sort()``` to sort the vector from small to large in place

---

### String
a collection of bytes and provides a representation for text

both String and string slice are UTF-8 encoded

use ```String::new()``` to create a new empty String instance

use ```string.to_string()``` to create String instance according to string literal

use ```String::from(string)``` to create String instance according to string literal

use ```push_str(s)``` to append string literal to the tail, ```push(c)``` to append one character to the tail

use ```+``` operator to append string which implicitly call ```add(self, s: &str) -> String```

use macro ```format!()``` to take String as reference and return a String with ownership

use ```contains()``` to check if contains a substring

use ```replace(s, new_s)``` to replace substring and return a new String with ownership

use ```replace_range()``` to replace the current string slice with new substring in place

#### Internal representation
String is a wrap over ```Vector<u8>```, because character is encoded as bytes (u8) in the memory.

use ```chars()```, ```bytes()``` to get the corresponding iteration where we can 

 - for loop it
 - use ```next()``` to iter over
 - use ```nth()``` to jump to the corresponding position

use ```char_indices()``` is the iterator that returns char index (in byte unit) and the char itself

---

### HashMap
use ```use std::collections::HashMap``` to import

use ```HashMap<T_key, T_value>``` to specify type and ```HashMap::new()``` to init

both ownership and reference can serve as key and value

use ```insert()``` to insert a new entry with value or overwrite the old value

use ```get()``` to get Option<&T_value> with the reference of T_key

use ```get_mut()``` to get Option<&mut T_value> with the reference of T_keys

#### Entry
use ```entry(key)``` to own an entry enum whoes variant can be
 - VacantEntry : if not exist
 - OccupiedEntry : if exist

use ```to_insert(v)``` to perform insertion when VacantEnry, and return mutable reference

use ```and_modify(func)``` to perform modify function

---

### Panic
use ```panic!(text)``` to abort the program with text shown, it's like exception thrown without try and deals in python. This macro returns something.

we can specify if panic cleanup memory ot not in Cargo.toml. Memory is cleaned up by default when panic
```
[profile.release]
panic = 'abort' # abort without cleanup
```

use environment variable to give more stack information when panic
```
// for powershell
$env:RUST_BACKTRACE=1; cargo run
$env:RUST_BACKTRACE='full'; cargo run
```

If there is no reason to fail (violate contract) or no way to recover or lead to insecure (like visit invalid memory), call ```panic!```

### Result
Result is an enum which is either ```Ok(T)``` or ```Err(E)```, ```std::io::Error``` can be one instance of E.

use ```unwrap_or_else()``` to return T directly or return from an error callback

use ```expect()``` to deal with ok branch only and leave specific error msg

error can be propagated by wrap data in Result

use ```?``` operator after result to unwrap the value if ok or return err from the whole function; the ```?``` operator can only be used in a function that returns ```Result``` or ```Option``` (or another type that implements ``FromResidual```)

If there can possibly exist error, return result instead of call ```panic!```

---

### Generic Type
abstract functionality that is capable for a group of data types

use ```<T, ...>``` behind the identifier as the syntax to generalize type to a specific group

generic type for impl is valid over the whole scope, to precisely restrict the method with certain traits, we can group method with multiple traits with different generic types

since enum type always has one variant valid at one time, why cannot we always use one generic type T? This is probably because we would like the type of different variant has different features. Remember that, generic type defines a set of valid type, if we would like the type set to be different, we have to decouple them into two types.

Rust will not select the more specified typed method among the same names (as cpp), rust treats this as ambiguous and rejects

Like cpp, monomorphization is performed in compile time to revert generic type to specific inferred type.

---

### Trait
trait define shared behavior within different types

to ensure certain functionality of generic type T, we should use trait bound to convince the compiler such that the interface of T is sufficient

use ```trait Trait {...}``` to includes a couple of method signatures inside. the implementation is not necessary. We can implement the interface with default definition. impl trait for type support override.

use ```trait Trait<T> {...}``` to have a trait of generic type

method signature ```impl Trait``` means some type that has Trait. However, it is still under the syntax of generic type. In other word, impl Trait will be monomorphized to one specific type in compile time, which the compiler does not allow different types.

use ```fn func<T>(x: &impl Trait<T>)``` to utilize generic trait as trait bound.

---

### Lifetime
lifetime of reference is inferred implicitly in most cases, however, we can declare it explicitly.

lifetime of reference is mainly to avoid dangling pointer (pointer's lifetime is larger than its value's lifetime)

generic lifetime uses lifetime annotation ```'a``` to relate parameters with each other: parameters with the same annotation share the same length of lifetime. Generic lifetime 'a means the compiler will find common lifetime such that to the function, the parameter decorated with same generic lifetime will end the lifetime at the same point.

lifetime syntax is about connecting the lifetimes of various parameters and return values of functions

whenever there is reference declaration in api, lifetime annotation is required. Rust decouples api compilation check with implementation compilation check.

lifetime elision is a syntax sugar for missing lifetime annotation. The three rules are as follow:
 - The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
 - The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
 - The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. 

#### elided lifetime
use ```'_``` to denote elided lifetime which will be inferred by the compiler with the specific lifetime. it is used for specify generic lifetime

#### static lifetime
use ```'static``` to denote static lifetime which is as long as the program's lifetime

---

### test
by ```cargo new xxx --lib```, a test mod is generated where cargo test can be implimented

beyond compilation check, we would like to check the functionality of the methods
 - passed
 - failed
 - ignored: specify by #[ignore], useful for some expensive test
 - measured
 - filtered out: specify by [test_func_substring] filter

except for assert in the test function, we can return __Result__ for test function

use ```#[should_panic(expected = "substring")]``` to expect a fail with substring match with panic msg, to use this feature, the test function must return ```()```

module decorated with ```#[cfg(test)]``` is only compiled in cargo test, but not cargo build

#### Test functions
cargo test recursively run functions under all mod starting from library root crate decorated with ```#[test]```

tests/ puts public test rs

test functions in lib / module crates are tested first (unit test - private functions), followed by binary crates, followed by tests/xx.rs (integration test - public functions), finalized by doc test. fail in any stage will block the following stages

#### Parser in command
 - ```--help```: get help information
 - ```--test-thresad```: the number of threads to run tests, by default not 1
 - ```--show-output```: show print outputs anyway. by default, only fail test outputs 
 - ```--ignored```: test ignored only
 - ```--include-ignored```: test normal with ignored
 - ```--test```: specify taget rs
 - ```cargo test [test_func_substring]``` : run test funcs matching with the substring

#### Test Attributes:
- ```#[test]``` - Indicates a function is a test to be run. This function takes no arguments.
- ```#[bench]``` - Indicates a function is a benchmark to be run. This unction takes one argument (test::Bencher).
- ```#[should_panic]``` - This function (also labeled with ```#[test]```) will only pass if the code causes a panic (an assertion failure or panic!) A message may be provided, which the failure string must contain: #[should_panic(expected = "foo")].
- ```#[ignore]``` - When applied to a function which is already attributed as a test, then the test runner will ignore these tests during normal test runs.

---

### Enrivonment Variable
use ```std::env::var()``` to get certain environment variables. If exists, the type is ```String```.

# Toolchain

### println!
{} tries to call Display function.

{:?} tries to display a debug mode

{:#?} 

### dbg!
give information of the value and line to stderr

