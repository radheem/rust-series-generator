pub fn print_type_of<T>(_: &T) {
    // pass a refernce to the object, variable etc
    println!("{}", std::any::type_name::<T>())
}
