fn main() {
// you can basically declare variables with the keyword 'let', like in JavaScript
    let name = "Brachyura";
    let fav_expression = "Shaw !!";
    let mut age = 21; // the 'mut' keyword allows you to change the value of a variable
    let weight = 2.72; // Otherwise you cannot change it, like this variable for exemple
    let species = "Brachyura";
    let description = "Brachyuran crabs are generally covered with a thick exoskeleton, \ncomposed primarily of highly mineralized chitin. \nBehind their pair of claws are six walking legs \nand then two swimming legs. \nThe crab breathes through gills on its underside; \ngills must be at least moist to work.";
    println!("{} is a {} (big crab). \nHe weighs approximately {} kg,\nand is {} years old.\nHis favorite expression is ''{}''.\n\n\n{}", name, species, weight, age, fav_expression, description);
    age = 22; // the age value is changed
    println!("\nOh, and he will be {} next year.", age);
    println!("   _      _\n  (<      >)\n   `O,99,O`\n  //-\\__/-\\\\  Crab.");
}