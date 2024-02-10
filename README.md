# RustToLearn
Rust learning progress

## Syntax

### Hello world:
use main as the code entry

use 4 spaces as indent

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

### Variables and mutability
create immutable variable (let is immutable by default).
```let x = 5```

create mutable variable
```let mut x = 5```

constant: 1. cannot use with mut; 2. type must be declared; 3. statically defined (not runtime)
```const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;```

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

### Statements and expressions
Statements are instructions that perform some action and do not return a value, like ```let y = 6;```

Expressions evaluate to a resultant value.

### Function:
declare function with the following format
```fn func_name(par1: type1, par2: type2) {statements;}```

function with return value. Explicit return using ```return``` statement or implicit return the final expression
```fn func_name(par1: type1, par2: type2) -> return_type {statements; expression}``` 



# Toolchain
