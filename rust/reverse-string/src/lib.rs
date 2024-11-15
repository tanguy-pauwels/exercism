pub fn reverse(input: &str) -> String  {
     input
         .chars() // Extract caractère from the str into an iterator
         .rev() // Reverse chars
         .collect::<String>() // Reassemble iterator's element into a String  
}
