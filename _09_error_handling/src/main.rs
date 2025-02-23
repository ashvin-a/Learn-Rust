fn main() {
    a();
}

fn a(){
    b();
}

fn b(){
    panic!("An error happened here");
}



