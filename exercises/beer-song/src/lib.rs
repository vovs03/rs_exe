pub fn verse(n: u32) -> String {
    // todo!("emit verse {n}")
    // #[warn(unreachable_code)]
    match n {
    	3..=99 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1),
    	2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
    	1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
    	0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
    	// 100_u32..=u32::MAX => panic!("invalid verse number")
    	_ => panic!("Input correct number")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    // todo!("sing verses {start} to {end}, inclusive")
    (end..=start).rev().map(|n| verse(n)).collect::<Vec<String>>().join("\n") 
}
