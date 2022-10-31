pub fn ptype<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
