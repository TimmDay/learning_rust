mod stacksize;

fn main() {
    let mut counter: usize = 0;
    stacksize::infinite_recursor(counter);
}