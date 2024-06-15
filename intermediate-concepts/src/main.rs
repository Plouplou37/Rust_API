mod module_test;
use module_test::{
    error_handling_example_1, error_handling_example_2, os_command_example_1, os_command_example_2,
};

fn main() {
    //module_test::os_command_example_1();
    //os_command_example_1();
    //module_test::os_command_example_2();
    //os_command_example_2();
    error_handling_example_2("src");
    error_handling_example_2("lib");
    //error_handling_example_2("--j");
}
