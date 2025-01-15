
pub fn slice(){
    let s2 = "hello world";
    let s3 = &s2[..];
    let s4 = &s2[..4];
    println!("{}, {}", s3, s4);

    let arr = [1,2,3];
    let sub_arr = &arr[..2];

    println!("{:?}", sub_arr);

    let mut s = String::from("hello world");
    let s2 = "hello world";
    let first_ = first_word(s2);

    println!("{}", first_);

}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]; 
        }
    }
    &s[..]
}