
// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Color2(u8,u8,u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name:first.to_string(),
            last_name:last.to_string()
        }
    }
    fn get_full_name(&self) -> String {
        format!("{} {}",self.first_name,self.last_name)
    }
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String,String) {
        (self.first_name,self.last_name)
    }
}


pub fn run() {
    let mut c = Color {
        red:255,
        green:255,
        blue:0
    };

    c.blue = 100;

    println!("Color: {} {} {}",c.red,c.green,c.blue);

    let mut b = Color2(255,255,0);

    b.2 = 100;

    println!("Color: {} {} {}", b.0,b.1,b.2);

    let mut p = Person::new("John","Doe");

    println!("Person: {} {}",p.first_name,p.last_name);

    p.set_last_name("White");
    println!("Person: {}",p.get_full_name());

    println!("Tuple: {:?}",p.to_tuple()); 
}