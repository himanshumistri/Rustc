mod person;

//Here is how you can define const in Rust program
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

fn main() {
    println!("Hello, world!");
    let myName =String::from("Himanshu Mistri");

    let myAge = 34;

   // We should not pass Direct variable but we should be pass by reference using &
    greet(&myName,&myAge);

    //Create struct object and print value from it
    create_person();
}

fn greet(name: &str,age:&u32) {
    //
    println!("Hello, {}! I am {}! year old", name,age);
    //Here is another way to print same output as well
    println!("Hello, {name}! I am {age}! year old");
    //Print const value here
    println!("My value is from const, 3 hour contain {THREE_HOURS_IN_SECONDS} seconds");
}

fn create_person(){

    let himanshu_mistri = person::Person::new("Himanshu", "Mistri", person::Gender::Male);
    println!("Person created , Name is {} \n End of Print",himanshu_mistri.get_full_name());

}
