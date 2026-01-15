// This function demonstrates VARIABLES and MUTABILITY in Rust
// It will be called from main.rs
pub fn exercise_variables_and_mutability() {

  // ------------------------------------
  // STEP 1: IMMUTABLE VARIABLE
  // ------------------------------------

  // By default, variables in Rust are immutable
  // Once assigned, the value cannot be changed
  let x = 10;

  // Printing the value of x
  println!("x = {}", x);

  // ------------------------------------
  // STEP 2: TRY TO REASSIGN x (ERROR CASE)
  // ------------------------------------

  // Uncomment the next line to see a COMPILER ERROR
  // Rust will not allow reassignment of immutable variables

  // x = 20; // ❌ ERROR: cannot assign twice to immutable variable `x`


  // ------------------------------------
  // STEP 3: MUTABLE VARIABLE
  // ------------------------------------

  // Using `mut` allows the value to be changed
  let mut y = 15;

  // Printing the initial value of y
  println!("y before change = {}", y);


  // ------------------------------------
  // STEP 4: REASSIGN MUTABLE VARIABLE
  // ------------------------------------

  // Since y is mutable, reassignment is allowed
  y = 25;

  // Printing the updated value of y
  println!("y after change = {}", y);
}