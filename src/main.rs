mod print_name;
mod mutable_variables;
mod datatypes;
mod type_conversion;
mod if_condition;
mod infinite_loop;
mod match_pattern;
mod arrays;
mod slice;
fn main() {
    print_name::print();
    mutable_variables::change_and_print_variables();
    datatypes::print_datatypes();
    type_conversion::convert_and_print();
    if_condition::make_decision();
    infinite_loop::run_loop_and_break();
    match_pattern::match_working_age();
    match_pattern::match_season();
    match_pattern::match_option();
    match_pattern::match_result();
    match_pattern::match_if_let();
    arrays::loop_through_array();
    slice::make_a_slice();
}