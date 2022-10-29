// fn main() {

//     let mut a = "ab".to_string();
//     let reference_to_nothing = dangle(&a);
// }
// fn dangle(x: &str) -> &str {
//     x
// }


// fn main() {
//     let reference_to_nothing = dangle();
// }
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }








// use std::collections::HashMap;

// fn main() {
//     let x = get_hash_map();
//     println!("{:?}", x);
// }

// fn get_hash_map() -> Option<&'static Vec<i32>> {
//     let mut hm = HashMap::new();
//     let mut vec = Vec::new();
//     vec.push(1);
//     hm.insert("1".to_string(), vec);
//     return hm.get("1");
// }


// use std::collections::HashMap;

// fn main() {
//     let mut hm = HashMap::new();
//     let x = get_hash_map(&mut hm);
//     println!("{:?}", x);
// }

// // note that both `hm` and the reference in the return type have the same 'a lifetime.
// fn get_hash_map<'a>(hm: &'a mut HashMap<String, Vec<i32>>) -> Option<&'a Vec<i32>> {
//     let mut vec = Vec::new();
//     vec.push(1);
//     hm.insert("1".to_string(), vec);
//     return hm.get("1");
// }


// struct Point<T,U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     // DON'T modify this code.
//     //let p = Point{x: 5, y : "hello".to_string()};
//     let p = Point::<i32,String>{x: 5, y : "hello".to_string()};
//     println!("Success!");
// }
// use std::ops::Add;

// //Implement the generic function below.
// fn sum<AB:std::ops::Add + Add<Output = AB>>(x: AB, y:AB) -> AB {
//     x+y
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }


// fn main() {
//     let a = A {p: Some("p".to_string())};
//     a.a();
// }

// struct A {
//     p: Option<String>
// }

// impl A {
//     fn a(self) -> Self {
//         Self::b(&self.p);
//         //&Option<String> -> Option<&String> -> &String
//         self
//     }
//     fn b(b: &Option<String>) {
//         print!("b: {:?}", b)
//     }
// }



// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }



// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }


// impl MyData {
//     pub fn get_val1(&self) -> i32 {
//         return self.val1;
//     }

//     pub fn get_val2(self) -> String {
//         return self.val2;
//     }

//     //self-> String 
//     //&self -> &String
//     pub fn get_both(&self) -> (i32, String) {
//         return (self.val1, self.val2.to_string());
//     }
// }




// #![allow(dead_code)]

// struct Store {
//     name: String,
//     items: Vec<Item>,
// }

// #[derive(Debug)]
// struct Item {
//     name: String,
//     price: f32,
// }

// impl Store {
//     fn new(name: String) -> Store {
//         Store {
//             name: name,
//             items: vec![],
//         }
//     }

//     fn add_item(&mut self, item: Item) {
//         self.items.push(item);
//     }

//     fn price(&self, item_name: &str) -> Option<f32> {
//         for item in &self.items {
//             if item.name == item_name {
//                 return Some(item.price);
//             }
//         }
//         None
//     }
//     //slice array [ slice string (string literal)]
//     fn total_price(&self, shopping_list: &[&str]) -> Option<f32> {
//         // Goal: compute the total price of all items in the shopping
//         // list. If any of the options are not present, return `None`.
//         //
//         // Hint: If you'd like a hint as to how to proceed, open
//         // <http://rust-tutorials.com/exercises/hint-struct-1.html>.

//         // let mut res = 0f32 ;
//         // for item in shopping_list.iter(){

//         //     self.items.iter().any(|i|i.name == item.to_string());
//         // }

//         // Some(res)

//         let mut res = 0f32;
//         let mut include_count = 0;
//         for item in self.items.iter() {
//             if shopping_list.contains(&item.name.as_str()) {
//                 res += item.price;
//                 include_count += 1;
//             }
//         }
//         if include_count == shopping_list.len() {
//             Some(res)
//         } else {
//             None
//         }
//     }
        

//     }


// fn build_store() -> Store {
//     let mut store = Store::new(format!("Rustmart"));
//     store.add_item(Item { name: format!("chocolate"), price: 5.0 });
//     store.add_item(Item { name: format!("socks"), price: 23.0 });
//     store.add_item(Item { name: format!("plush Mozilla dinosaur"), price: 13.0 });
//     store
// }


// // unit test
// #[test]
// fn total_price() {
//     let store = build_store();
//     let list = vec!["chocolate", "plush Mozilla dinosaur"];
//     assert_eq!(store.total_price(&list), Some(18.0));
// }

// #[test]
// fn total_price_missing() {
//     let store = build_store();
//     let list = vec!["chocolate", "cacao", "plush Mozilla dinosaur", "fork and knife"];
//     assert_eq!(store.total_price(&list), None);
// }


// #[test]
// fn total_price_empty() {
//     let store = build_store();
//     let list = vec![];
//     assert_eq!(store.total_price(&list),None);
// }




//generic type
fn test<T> (x:T) {}
//generic parameter
fn test1<'a> () {}

// fn main() {
//     let s = "ab";
//     let reference_to_nothing = dangle(s);
// }
// fn dangle(s:&str) -> &String {
//     //let s = String::from("hello");
//     let res = s.to_string();
//     &res
// }
// fn main() {
//     let s = "ASD";
//     let ref_to = dangle(s);
//     println!("{}", ref_to);
// }

// fn dangle(s: &str) -> &str {
//     s
// }


// borrow checker

// fn main(){
//     let r; 
//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("{}",r);
// }






// fn main() {
//     let reference_to_nothing = dangle();
// }
// fn dangle() -> &'static String {
//     let s = String::from("hello");
//     &s
// }



// fn main() {
//     let string1 = "abcd";
//     let string2 = "xyz";
//     let result = longest(string1.to_string(), string2.to_string());

//     println!("string:{}",string1);
//     println!("The longest string is {}", result);
// }


// fn longest<'a>(x:String , y:String ) -> &str {
//     if x.len() > y.len() {
//         x.as_str()
//     } else {
//         y.as_str()
//     }
// }



// fn main(){

// }

// fn long(z:&str) -> &str {
//     if z.len() > 0{
//         z
//     }

//     else {
//         "ok"
//     }
// }



// struct A {
//     name:String,
//     age: u8,
// }

// impl A {
//     fn test();
//     fn foo();
// }



// enum C{
//     name,
//     age,
// }

// impl C {
//     fn test();
//     fn foo();
// }


// trait Test {
// //     fn test();
// //     fn foo();
// }

//Test -> u32
// Test -> struct



//interface

// định nghĩa các hành vi chung -> mình ko implement cụ thể 
// trait Speak {
//     fn say_hello(&self) -> String;
//     fn language(&self) -> String{ 
//         "Vietnamese".to_string()
//     }
//     //fn say_hello(&mut self) -> String;
// }



// impl Struct 
// impl trait for Struct 

// design pattern , design code 
// khác biệt : cần impl hết các fn trong trait
// struct Person {}
// impl Speak for Person {

//     // implement
//     fn say_hello(&self) -> String {
//         String::from("Hello!")
//     }

// }

// struct Cat{}

// impl Speak for Cat {
//     fn say_hello(&self) -> String {
//         String::from("Meo Meo")
//     }
// }

// impl Speak for u32 {
//     fn say_hello(&self) -> String {
//         String::from("u32")
//     }
// }


// fn main(){

//     let person1 = Person{};
//     let res = person1.say_hello();
//     let res1 = person1.language();

// }



// struct Test {}

// impl Test {
//     fn say_hello() -> String {
//         String::from("OK")
//     }
// }

//Test::say_hello()


// trait Speak {
//     fn say_hello(&self) -> String;
// }

// // trait as parameter
// fn give_greeting(p: impl Speak) {
//     println!("{}", p.say_hello());
// }

// struct Cat{}

// impl Speak for Cat {
//     fn say_hello(&self) -> String {
//         String::from("Meo Meo")
//     }
// }


// struct Person {}
// impl Speak for Person {

//     // implement
//     fn say_hello(&self) -> String {
//         String::from("Hello!")
//     }

// }

// fn main(){
//     let cat = Cat{};
//     let perr = Person{};
//     give_greeting(cat);
//     give_greeting(perr);
// }

// trait bound




// trait Speak {
//     fn say_hello(&self) -> String;
// }


// trait Listen {
//     fn listen(&self) -> String;
// }

// // generic type 
// fn give_greeting<T: Speak+ Listen>(p: T) {
//     println!("{}", p.say_hello());
// }

// struct Cat{}

// impl Speak for Cat {
//     fn say_hello(&self) -> String {
//         String::from("Meo Meo")
//     }
// }


// impl Listen for Cat {
//     fn listen(&self) -> String {
//         String::from("Meo")
//     }
// }

// struct Person {}
// impl Speak for Person {

//     // implement
//     fn say_hello(&self) -> String {
//         String::from("Hello!")
//     }

// }

// fn main(){
//     let cat = Cat{};
//     let perr = Person{};
//     give_greeting(cat);
//     //give_greeting(perr);
// }






trait Speak {

    //signature method
    fn say_hello(&self) -> String;
}

trait Talk {
    fn say_hello(&self) -> String;
}

// generic type 
fn give_greeting_speak<T: Speak+ Talk>(p: T) {

    println!("{}",<T as Speak>::say_hello(&p));
}


fn give_greeting_talk<T: Speak+ Talk>(p: T) {

    println!("{}",<T as Speak>::say_hello(&p));
}


struct Cat{}

impl Speak for Cat {
    fn say_hello(&self) -> String {
        String::from("Meo Meo")
    }
}


impl Talk for Cat {
    fn say_hello(&self) -> String {
        String::from("Meo")
    }
}

struct Person {}
impl Speak for Person {

    // implement
    fn say_hello(&self) -> String {
        String::from("Hello!")
    }

}

fn main(){
    let cat = Cat{};
    let perr = Person{};
    give_greeting_speak(cat);
    //give_greeting_talk(cat);
    //give_greeting(perr);
}



