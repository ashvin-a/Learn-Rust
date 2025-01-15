

pub fn first_word(s: &String) -> usize{

    let bytes = s.as_bytes();

    for (i, &val) in bytes.iter().enumerate(){
        if val == b' '{
            return i;
        }
    }
    
    s.len()
}