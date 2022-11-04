// use std::io;
// fn main() {
//     let mut input: Vec<String>;

//     loop {
//         let mut input_text = String::new();
//         println!("Type instruction in the format Add <name> to <department>:");
//         io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
//         let trimmed_text: String; 
//         trimmed_text = input_text.trim().to_string();

//         input = trimmed_text.split(" ").map(String::from).collect();
//         if input[0] == "Add" && input[2] == "to" {
//             break;
//         } else {
//             println!("Invalid format.");
//         }


//     }
//     //println!("{:?}", input);
// }



// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for String {
//     //Add your code here
//     fn append_bar(mut self) -> String{
//         self.push_str("Bar");
//         self
//         //self +"Bar"
//         //format!("{}Bar",self)
//     }

// }

// fn main() {
//     let s = String::from("Foo");
//     let s = s.append_bar();
//     println!("s: {}", s);
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_foo_bar() {
//         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
//     }

//     #[test]
//     fn is_bar_bar() {
//         assert_eq!(
//             String::from("").append_bar().append_bar(),
//             String::from("BarBar")
//         );
//     }
// }



// trait AppendBar {
//     fn append_bar(self) -> Self;
// }
// //TODO: Add your code here
// impl AppendBar for Vec<String> {
//     fn append_bar(mut self) -> Self{

//         //let mut s = self;
//         self.push(String::from("Bar"));
//         self
//         //s
        
//     }


// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_vec_pop_eq_bar() {
//         let mut foo = vec![String::from("Foo")].append_bar();
//         //vec!["Foo","Bar"]
//         assert_eq!(foo.pop().unwrap(), String::from("Bar"));
//         assert_eq!(foo.pop().unwrap(), String::from("Foo"));
//     }
// }




//////////////////////////////////////////////////////////////////////////////
///The trait
//////////////////////////////////////////////////////////////////////////////





// struct Struct;

// trait Generic<G> {
//     fn generic(&self, generic: G);
// }

// impl<G> Generic<G> for Struct {
//     fn generic(&self, _: G) {}
// }

// fn main() {
//     Struct.generic(1);
//     Struct.generic("a");
// }



// fn main(){

//     let x = 3;
//     if x ==5 {
//         println!("Ok");
//     }
//     else {
//         panic!("Message");
//     }

//     let y = 10;
//     println!("Hello :{}",y);


// }

// use std::fs::File;
// use std::io::prelude::*;

// use std::default::Default;


// pub struct MyFile;

// impl MyFile {
//     fn open(path: &str) -> MyFile{

//         Self
//     }


    
// }

// impl Default for MyFile {
//     fn default() -> Self{
//         Self
//     }
// }

// fn main(){
//     // xoá Result
//     //let mut file = File::open("foo.txt").expect("Can not open file ");
//     // ? => unwrap 
//     let mut file = File::open("foo.txt");

//     // match file {

//     //     Ok(_) => println!("Success"),
//     //     Err(_) => {

//     //         let mut new_file = File::create("foo.txt").unwrap();
//     //         new_file.write_all(b"Hello, world!");

//     //     }
//     // }

//     // () unit type
// }


// pub fn generate_nametag_text(name: String) -> Option<String> {
//     if name.is_empty() {
//         // Empty names aren't allowed.
//         None
//     } else {
//         Some(format!("Hi! My name is {}", name))
//     }
//  }
 
//  #[cfg(test)]
//  mod tests {
//     use super::*;
//     #[test]
//     fn generates_nametag_text_for_a_nonempty_name() {
//         assert_eq!(
//             generate_nametag_text("Beyoncé".into()).ok_or("`name` was empty; it must be nonempty."),
//             Ok("Hi! My name is Beyoncé".into())
//         );
//     }
 
 
//     #[test]
//     fn explains_why_generating_nametag_text_fails() {
//         assert_eq!(
//             generate_nametag_text("".into()).ok_or("`name` was empty; it must be nonempty."),
//             // Don't change this line
//             Err("`name` was empty; it must be nonempty.".into())
//         );
//     }
//  }
 


// use std::num::ParseIntError;


// pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
//    let processing_fee = 1;
//    let cost_per_item = 5;
//    let qty = item_quantity.parse::<i32>()?;
   
//    Ok(qty * cost_per_item + processing_fee)
// //    match qty {
// //     Ok(val) => return Ok(val * cost_per_item + processing_fee),
// //     Err(e) => return Err(e)
// //     }
   
// }


// #[cfg(test)]
// mod tests {
//    use super::*;


//    #[test]
//    fn item_quantity_is_a_valid_number() {
//        assert_eq!(total_cost("34"), Ok(171));
//    }


//    #[test]
//    fn item_quantity_is_an_invalid_number() {
//        assert_eq!(
//            total_cost("beep boop").unwrap_err().to_string(),
//            "invalid digit found in string"
//        );
//    }
// }



// use std::num::ParseIntError;


// fn main() -> Result<(), &'static str> {
//    let mut tokens = 100;
//    let pretend_user_input = "8";


//    let cost = total_cost(pretend_user_input).expect("Can not get");


//    if cost > tokens {
//        println!("You can't afford that many!");
//    } else {
//        tokens -= cost;
//        println!("You now have {} tokens.", tokens);
//    }

//    Ok(())
// }


// pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
//    let processing_fee = 1;
//    let cost_per_item = 5;
//    let qty = item_quantity.parse::<i32>()?;


//    Ok(qty * cost_per_item + processing_fee)
// }





// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);


// #[derive(PartialEq, Debug)]
// enum CreationError {
//    Negative,
//    Zero,
// }


// impl PositiveNonzeroInteger {
//    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//        // Hmm...? Why is this only returning an Ok value?
//        if value <0 {
//             return Err(CreationError::Negative)
//         }
//         else if value ==0 {
//             return Err(CreationError::Zero)
//         }
//        Ok(PositiveNonzeroInteger(value as u64))
//    }
// }


// #[test]
// fn test_creation() {
//    assert!(PositiveNonzeroInteger::new(10).is_ok());
//    assert_eq!(
//        Err(CreationError::Negative),
//        PositiveNonzeroInteger::new(-10)
//    );
//    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
// }



// use std::error;
// use std::fmt;
// use std::num::ParseIntError;


// // TODO: update the return type of `main()` to make this compile.
// fn main() -> Result<(), Box<dyn error::Error>> {
//    let pretend_user_input = "a";
//    let x: i64 = pretend_user_input.parse()?;
//    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
//    Ok(())
// }


// // Don't change anything below this line.


// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);


// #[derive(PartialEq, Debug)]
// enum CreationError {
//    Negative,
//    Zero,
// }


// impl PositiveNonzeroInteger {
//    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//        match value {
//            x if x < 0 => Err(CreationError::Negative),
//            x if x == 0 => Err(CreationError::Zero),
//            x => Ok(PositiveNonzeroInteger(x as u64))
//        }
//    }
// }


// //This is required so that `CreationError` can implement `error::Error`.
// impl fmt::Display for CreationError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let description = match *self {
//            CreationError::Negative => "number is negative",
//            CreationError::Zero => "number is zero",
//        };
//        f.write_str(description)
//    }
// }


// impl error::Error for CreationError {}




// use std::num::ParseIntError;


// // This is a custom error type that we will be using in `parse_pos_nonzero()`.
// #[derive(PartialEq, Debug)]
// enum ParsePosNonzeroError {
//    Creation(CreationError),
//    ParseInt(ParseIntError)
// }


// impl ParsePosNonzeroError {
//    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
//        ParsePosNonzeroError::Creation(err)
//    }
//    // TODO: add another error conversion function here.
//    fn from_parseint(err:ParseIntError ) -> ParsePosNonzeroError{
      
//       ParsePosNonzeroError::ParseInt(err)
// }
// }


// fn parse_pos_nonzero(s: &str)
//    -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
// {
//    // TODO: change this to return an appropriate error instead of panicking
//    // when `parse()` returns an error.
//    let x = s.parse().map_err(|e|ParsePosNonzeroError::from_parseint(e))?;
//    PositiveNonzeroInteger::new(x)
//        .map_err(ParsePosNonzeroError::from_creation)
// }


// // Don't change anything below this line.


// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);


// #[derive(PartialEq, Debug)]
// enum CreationError {
//    Negative,
//    Zero,
// }


// impl PositiveNonzeroInteger {
//    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//        match value {
//            x if x < 0 => Err(CreationError::Negative),
//            x if x == 0 => Err(CreationError::Zero),
//            x => Ok(PositiveNonzeroInteger(x as u64))
//        }
//    }
// }


// #[cfg(test)]
// mod test {
//    use super::*;


//    #[test]
//    fn test_parse_error() {
//        // We can't construct a ParseIntError, so we have to pattern match.
//        assert!(matches!(
//            parse_pos_nonzero("not a number"),
//            Err(ParsePosNonzeroError::ParseInt(_))
//        ));
//    }


//    #[test]
//    fn test_negative() {
//        assert_eq!(
//            parse_pos_nonzero("-555"),
//            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
//        );
//    }


//    #[test]
//    fn test_zero() {
//        assert_eq!(
//            parse_pos_nonzero("0"),
//            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
//        );
//    }


//    #[test]
//    fn test_positive() {
//        let x = PositiveNonzeroInteger::new(42);
//        assert!(x.is_ok());
//        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
//    }
// }



// use std::default::Default;
// // macro: code trong code 

// //Cách 1 : sử dụng derive Debug (macro)
// #[derive(Debug,Clone)]
// struct A {

//     test : String
// }

// impl Default for A {

//     fn default() -> Self{

//         A {
//         test:"WOlrd".to_string()}
//     }
// }

// fn main() {

//     let a = A {test:"Hello".to_string()};
//     let c = a.clone();
//     println!("{:?}",a);
    

//     let b = A::default();
//     println!("{:?}",b);

// }

// struct A {

//     test : String
// }


// impl std::fmt::Debug for A {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("A")
//          .field("test", &self.test)

//          .finish()
//     }
// }


// fn main() {

//     let a = A {test:"Hello".to_string()};

//     println!("{:?}",a);
// }






macro_rules! avec {
    () => {
        Vec::new()
    };

    ($ele:expr,$ele2:expr) => {{
        let mut temp = Vec::new();
        temp.push($ele);
        temp.push($ele2);
        temp
    }};
    ($($ele:expr),*) => {{
        let mut temp = Vec::new();
        $(temp.push($ele);)*
        temp
    }}


}

fn main(){

    //let v = vec![1,2,3];

    // vector rỗng
    let v:Vec<u32> = avec![1,2,3,4,5,6,7];

    println!("v:{:?}",v);


}
