mod print_name;
mod mutable_variables;
mod datatypes;
mod type_conversion;
mod infinite_loop;
mod conditions {
    pub mod match_pattern;
    pub mod if_condition;
}
mod data_structures {
    pub mod arrays;
    pub mod slice;
    pub mod tuples;
    pub mod structs;
}
fn main() {
    print_name::print();
    mutable_variables::change_and_print_variables();
    datatypes::print_datatypes();
    type_conversion::convert_and_print();
    conditions::if_condition::make_decision();
    infinite_loop::run_loop_and_break();
    conditions::match_pattern::match_working_age();
    conditions::match_pattern::match_season();
    conditions::match_pattern::match_option();
    conditions::match_pattern::match_result();
    conditions::match_pattern::match_if_let();
    data_structures::arrays::loop_through_array();
    data_structures::slice::make_a_slice();
    data_structures::tuples::create_and_print_tuple();
    data_structures::structs::print_a_struct();
}