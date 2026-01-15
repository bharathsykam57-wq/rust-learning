// This exercise demonstrates OWNERSHIP TRANSFER and BORROWING in Rust
// It will be called from main.rs

pub fn ownership_examples() {

  // ------------------------------------
  // STEP 1: Create a String and assign it to s1
  // ------------------------------------

  // String::from creates a heap-allocated String
  let s1 = String::from("hello");

  // s1 is the OWNER of the String "hello"


  // ------------------------------------
  // STEP 2: Transfer ownership from s1 to s2
  // ------------------------------------

  // Ownership of the String is MOVED from s1 to s2
  let s2 = s1;

  // s2 is now the OWNER of the String
  println!("s2 = {}", s2);


  // ------------------------------------
  // STEP 3: Attempt to use s1 after move
  // ------------------------------------

  // Uncomment the next line to see a COMPILER ERROR
  // s1 is no longer valid after ownership transfer

  // println!("s1 = {}", s1); // ❌ ERROR: value borrowed after move

 
  // ------------------------------------
  // STEP 4: Borrow s2 using an immutable reference
  // ------------------------------------

  // We pass a REFERENCE to s2, not ownership
  print_string(&s2);

  // s2 is still valid after borrowing
  println!("s2 after borrowing = {}", s2);
}


// ------------------------------------
// STEP 5: Function that BORROWS a String
// ------------------------------------

fn print_string(text: &String) {

  // `text` is an immutable reference (&String)
  // We can READ the data, but NOT modify it
  println!("Inside print_string(): {}", text);
}
  