// drive5.rs
// Your task is to make the testcase be able to call the `my_demo_function` in module Foo.
// the `my_demo_function_alias` is an alias for `my_demo_function_alias`, so the two line of
// code in the testcase should call the same function.
// You should not modify any existing code. All you need to do is add two line of attributes.


// 
extern {
    #[link_name = "my_demo_function"]
    fn my_demo_function(a: u32) -> u32;

    #[link_name = "my_demo_function_alias"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

// mod Adapter {
//     use super::*;

//     #[no_mangle]
//     pub extern "C" fn my_demo_function_adapter(a: u32) -> u32 {
//         Foo::my_demo_function(a)
//     }

//     #[no_mangle]
//     pub extern "C" fn my_demo_function_alias_adapter(a: u32) -> u32 {
//         my_demo_function_alias(a)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_success() {
//         unsafe {
//             my_demo_function(123);
//             my_demo_function_alias(456);
//         }
//     }
// }
