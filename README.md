# RustToLearn
Rust learning progress with toy examples for review and illustration.

## Syntax

### Hello world:
use main as the code entry

use 4 spaces as indent

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

Actually, in the syntax below, sequential pattern match is what actually done. Pattern is like the lambda in python, a pattern is matched if and only if there exist instantiation of variable in pattern such that pattern is equal to instance. That's why pattern _, which contains a variable only, is always matchable. 

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


# Toolchain

### println!
{} tries to call Display function.

{:?} tries to display a debug mode

{:#?} 

### dbg!
give information of the value and line to stderr

