const DAYS: [&str; 12]= ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];


const VERSE: [&str; 12] = ["Twelve drummers drumming,", 
                           "Eleven pipers piping,", 
                           "Ten lords a-leaping,", 
                           "Nine ladies dancing,", 
                           "Eight maids a-milking,", 
                           "Seven swans a-swimming,", 
                           "Six geese a-laying,", 
                           "Five gold rings,", 
                           "Four calling birds,", 
                           "Three French hens,", 
                           "Two turtle doves,", 
                           "And a partridge in a pear tree."];



fn print_std_lines(day: usize) {

    println!("On the {} day of Christmas,\nmy true love sent to me", DAYS[day-1]);
}


fn print_verse(day: usize) {
    if day == 1 {
        println!("A partridge in a pear tree.");
    } else {
        for v in &VERSE[(12-(day-1)-1)..] {
            println!("{}",v);
        }
    }
}

        



fn main() {
    println!("\n12 DAYS OF CHRISTMAS");
    for d in 1..13{
        print_std_lines(d);
        print_verse(d);
        println!("\n");
    }
}
