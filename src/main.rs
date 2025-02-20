mod print_name;
mod mutable_variables;
mod datatypes;
mod type_conversion;
fn main() {
    print_name::print();
    mutable_variables::change_and_print_variables();
    datatypes::print_datatypes();
    type_conversion::convert_and_print();
}
