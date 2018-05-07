
pub fn infinite_recursor(mut depth_recursion_counter: usize) {

    depth_recursion_counter += 1;

    //number of recursions * kb of array allocation (1) + approx consumed stack cap for println!
    println!("approximate stack size: {}", depth_recursion_counter + 708);

    let x = [0u8; 1000]; // ie allocate/consume 1kb of space of the stack for this array

    infinite_recursor(depth_recursion_counter)
}



//see question1-2.txt for a clearer discussion

//2.0 Discussion
//
//ulimit -s gives a result of 8192 kB on my mac.
//
//The program approximates stacksize by infinitely recursing a function, and allocating 1kb of stack space to an array each time.
//It also prints the approximation on each iteration.
//The original result was around 7180 kb
//
//Why the difference of ~1000 kB to the ulimit measure?
//I suspect that it is due to the things we are not counting during the recursions that are consuming stack space.
//
//These include the println! macro, and maybe some other minor operations.
//Indeed, when I included a second println! call, to estimate decreased from 7174 to 6466. So we can estimate that the println! consumes about 708 kB.
//
//Adding that 708 back gives us 7882.
//
//I ammended the program to add this 708 kB back, and then discovered that the '+' operation within the println! call also consumes some of the stack. It is a tricky thing to pin down.
//
//
//tldr;
//println! stack space consumption estimate :  708 kB
//initial program max stack space estimate :	7174 kB
//adjusted program max stack estimate :		7785 kB
//ulimit stack size information :				8192 kB
//assumed stack capacity consumed by other
//operations in the program
//(+, +=, x variable binding, function calls): 407 kB
//
//Final Program result : 7785 kB

