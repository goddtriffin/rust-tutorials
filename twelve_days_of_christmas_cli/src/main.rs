const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

const LYRICS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

fn main() {
    for (index, day) in DAYS.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", day);

        for lyric_index in {0..index+1}.rev() {
            println!("{}", LYRICS[lyric_index]);
        }

        if index < DAYS.len()-1 {
            println!();
        }
    }
}
