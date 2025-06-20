const NUM: &[&str; 11] =&["No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    (0..(take_down as usize)).map(|taken| {
    	let bottles: usize = (start_bottles as usize) - taken;
    	format!(
    		"\
{count} green bottle{multiple} hanging on the wall,
{count} green bottle{multiple} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {reduced} green bottle{reduced_multiple} hanging on the wall.
",
		count = NUM[bottles],
		multiple = if bottles > 1 { "s"} else { "" },
		reduced = NUM[bottles -1].to_lowercase(),
		reduced_multiple = if bottles == 2 { "" } else { "s" }
		)
    }).collect::<Vec<_>>().join("\n")
}
