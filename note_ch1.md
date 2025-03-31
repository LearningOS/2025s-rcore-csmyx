## chapter 1

### remove std-lib dependency

#### remove std support
Cause we want to develop our own os, so we can't depend on the `std` lib. so we add `#![no_std]` in front of main.rs to remove dependency on `std` lib. we only use the `core` lib, which is decouple from os (I think).

#### add panic_handler
We need a panic_handler to make the crab happy. It is support by `std`, but not `core`. So we add it manually in a mod `lang_items`. Then we remove the main function, and add `#![no_main]` in fron of main.rs too. Now we finally got a useless but complete program, that doesn't rely on std lib provided by os!

### build user space execution enviorment

#### support syscall
Implement syscall function by `ecall` in risc-v assembly.

#### support exit 
Call #93 of syscall, with exit state number as args0.

#### support print
Call #64 of syscall, with fd as args0, buffer ptr as args1, buffer len as args2. Then implement formatting macros: `print!` and `println!`. So we got our hello world program ourselves instead of std-lib!
