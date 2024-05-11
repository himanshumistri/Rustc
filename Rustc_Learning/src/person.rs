pub struct Person{
    first_name:String,
    last_name:String,
    gender: Gender
}

pub enum Gender {
    Male,
    Female,
    Other,
}

impl Person{
    pub fn new(first_name:&str,last_name:&str,gender:Gender)-> Person{
        Person{
            first_name : first_name.to_string(),
            last_name : last_name.to_string(),
            gender
        }
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}