mod print_name;
mod mutable_variables;
mod datatypes;
mod type_conversion;
mod if_condition;
mod infinite_loop;
fn main() {
    print_name::print();
    mutable_variables::change_and_print_variables();
    datatypes::print_datatypes();
    type_conversion::convert_and_print();
    if_condition::make_decision();
    // infinite_loop::run_inifinite_loop();
    
}