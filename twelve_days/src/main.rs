fn main() {
    // Array of the first 12 english ordinal numbers.
    let ordinals = ["first", "second", "third", "fourth", "fifth", "sixth",
                                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    // Array of the 12 gifts in the traditional song.
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three French hens",
                                "four calling birds", "five golden rings", "six geese a laying",
                                "seven swans a swimming", "eight maids a milking",
                                "nine ladies dancing", "ten lords a leaping",
                                "eleven pipers piping", "twelve drummers drumming"];

    // Check that the arrays are the same length.
    // I was hoping this worked. Neat!
    assert!(ordinals.len() == gifts.len());
    let num_days = ordinals.len();

    // Loop through the days of Christmas.
    for day in 0..num_days {
        println!("On the {} day of Christmas my, true love sent to me:", ordinals[day]);
        
        // If it's the first day, just print the first gift.
        if day == 0 {
            println!("{}.\n", gifts[0]);
        // If it's not the first day, print all the gifts for that day.
        } else {
            for gift in (0..day+1).rev() {
                if gift == 0 {
                    // Print will not add a newline until you add println!.
                    // Print "and" on the same line as the last gift.
                    print!("and ");
                }

                // Print the gift.
                print!("{}", gifts[gift]);

                // Print a comma after each gift except the last.
                if gift != 0 {
                    print!(", ");
                }
            }
            // Print a newline after the last gift so the output is easier to read.
            println!(".\n");
        }
    }
}
