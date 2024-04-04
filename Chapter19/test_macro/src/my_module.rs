pub mod my_module {

    // Using this to export the macro to the crate level, regardless of the pub of its parent module
    // As if this macro is directly added to the root crate
    #[macro_export]
    // macro_rules! create new macro with specified name
    macro_rules! my_vec { // follow armed pattern match, every single variable is in the format as $x:expr, $y:expr, ...
        ( $x:expr, $y:expr ) => { // should return a single expression
            {
                (($x)..($y)).collect()
            }
        };
        ( $( $x:expr );* ) => { // should return a single expression
            // if we remove this embrace as sub scope, the compiler will reject
            // imagine multiple macro definitions in the same scope
            // if we do not isolate macro with scope, after code replacement, the later macro will remove the previous data
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x); // reference of the variable in the body
                )*
                temp_vec
            }
        };
    }

    pub fn new_vector() -> Vec<i32> {
        my_vec!(1;2;3;4;5) // it does not matter, if we use () [] or {} after the macro!
    }

    pub fn new_vector_range(x:i32, y:i32) -> Vec<i32> {
        my_vec![x,y] // it does not matter, if we use () [] or {} after the macro!
    }
}