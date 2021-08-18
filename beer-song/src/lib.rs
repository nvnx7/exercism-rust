pub fn verse(n: u32) -> String {
    let line: String;

    match n {
        0 => line = format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => line = format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => line = format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => line = format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n - 1)
    }

    line
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses: Vec<String> = Vec::new();

    for n in (end..=start).rev() {
        verses.push(verse(n));
    }

    verses.join("\n")
}
