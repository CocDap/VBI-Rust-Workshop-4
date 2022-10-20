


//================================
//VÍ DỤ 1
//================================



fn main() {
    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn remove_vowels(name: String) -> String {
    // Goal #1: What is needed here to make this compile?
    let output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels
            }
            _ => {
                output.push(c);
            }
        }
    }
    output
}

fn print_out(name: String) {
    let devowelized_name = remove_vowels(name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // Goal #2: What happens when you uncomment the `println` below?
    // Can you change the code above so that the code below compiles
    // successfully?
    //
    // println!("Removing vowels from {:?} yields {:?}",
    //          name, devowelized_name);

    // Extra credit: Can you do it without copying any data?
    // (Using only ownership transfer)
}



//================================
//VÍ DỤ 2
//================================




// pub fn main() {
//     let string = format!("my friend");
//     greet(string.clone());
//     greet(string);
// }

// fn greet(name: String) {
//     println!("Hello, {}!", name);
// }

// // Goal #1: Convert `greet` to use borrowing, not ownership, so that
// // this program executes without doing any cloning.
// //
// // Goal #2: Use a subslice so that it prints "Hello, friend" instead of
// // "Hello, my friend".


//================================
//VÍ DỤ 3
//================================




// pub fn main() {
//     let (mut str1, str2) = two_words();
//     str1 = join_words(str1, str2);
//     println!("concatenated string is {:?}", str1);
// }

// fn two_words() -> (String, String) {
//     (format!("fellow"), format!("Rustaceans"))
// }

// /// Concatenate `suffix` onto the end of `prefix`.
// fn join_words(mut prefix: String, suffix: String) -> String {
//     prefix.push(' '); // separate the words with a space
//     for ch in suffix.chars() {
//         prefix.push(ch);
//     }
//     prefix
// }

// Challenge: Convert `join_words` to use borrowing, not ownership.
// The new function should mutate `prefix` in place, and should not
// take ownership of `suffix`.
//
// Hint: If you'd like a hint as to how to proceed, open
// <http://rust-tutorials.com/exercises/hint/mutable_borrow_1/>.

// Question: Now that you've converted `join_words`, what happens if you
// call `join_words` using the same string for `prefix` and `suffix`?
// Why?




//================================
//VÍ DỤ 4
//================================



// fn main() {
    
//     let x = change_value(10,&mut 20);
// }
// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input ==1 {
//         *output =3;
//     }
//     else {
//         *output = 4;
//     }

//     *output
// }


//================================
//VÍ DỤ 5
//================================



// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 2;
//         if vector_is_prime(num, &primes) {
//             count += 1;
//             primes.push(num);
//         }
//     }
//     println!("{:?}", primes);

//     let array = &[10,20];
// }
// // p = &vec[1,2,3,4] -> &1, &2, &3

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
//     for &i in p {
//         if num > i && num % i != 0 {
//             return false;
//         }
//     }

//     true
// }

//================================
//VÍ DỤ 6
//================================


// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;
//     //let v1 = &v;
    
//     for n in v.iter() {
//         max = std::cmp::max(max, *n);
//     }
//     //println!("{:?}",v);
//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
    
//     for n in v {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }