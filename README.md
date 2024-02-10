# RustToLearn
Rust learning progress

## Syntax

### hello_world:
use main as the code entry

use 4 spaces as indent

### hello_cargo:
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

# Toolchain
