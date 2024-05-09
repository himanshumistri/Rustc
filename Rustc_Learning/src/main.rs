fn main() {
    println!("Hello, world!");
    let myName =String::from("Himanshu Mistri");

    let myAge = 34;

   // We should not pass Direct variable but we should be pass by reference using &
    greet(&myName,&myAge);
}

fn greet(name: &str,age:&u32) {
    println!("Hello, {}! I am {}! year old", name,age);
}
