pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let num_string = num.to_string();
    num_string.chars()
    	.map(|c| u128::pow(c as u128 - 48_u128, num_string.len() as u32))
    	.sum::<u128>() == num as u128 
}
