fn main() {
    //rust auto declares the datatype
    let name = "Saikat";
    let age = 21;
    let language = "Rust";


    /*
    also its immutable by default you cant cahnge there value after declaration
    let age = 21;
    // This will cause an error:
    age = 22;
     */

    /*
    for really change a value we have to use the mut
    fn main() {
    let mut age = 21;

    age = 22;

    println!("Age: {}", age);
}
     */

    /*
        A few important distinctions:
                - i32 = signed 32-bit integer.
                - u32 = unsigned 32-bit integer.
                - f32 = 32-bit floating-point number.
                - f64 = 64-bit floating-point number.
                - &str = borrowed string slice.
                - String = owned, growable string.          
     */

    println!("Name: {}",name);
    println!("Age: {}",age);
    println!("Language: {}",language);

}
