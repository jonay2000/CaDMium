
fn xauth() {

}


pub fn start_x() {
    println!("{}", std::env::var("XAUTHORITY").expect("no xauth"))
}
