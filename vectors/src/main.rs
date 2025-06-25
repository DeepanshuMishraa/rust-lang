//Vectors => Vectors are basically dynamic arrays , their size can be changed at runtime



fn main() {
    let mut years:Vec<i32> = vec![1,2,3,4];
    years.push(2010); // now years has 5 elements , ending with 2010
    years.push(2011);  // now years has 6 elements  ,ending with 2011

    println!("Number of years : {}",years.len());

    //usize => lenght of vec returns a usize. its size variaes depending upon what system you are compiling it on. if compiling on 32bit system it is equivalent to u32 and same for 64bit system equivalent to a u64

    let length : usize  = years.len();

    let mut arr : [u8,3] = [1,2,3];
    let mut vecs : Vec<u8> = vec![1,2,3];

    for num in nums{

    }

    // the tradeoff here sets the stage for the biggest factor in language performance.


    /*
    Stack Memory
    // Stack memory is memory that is automatically managed by the program
    // - Memory is allocated when variables are declared
    // - Memory is deallocated when variables go out of scope
    // - Size must be known at compile time
    // - Fast access and deallocation
    // - Limited in size
    
    // Example of stack allocation:
    let x = 5; // Integer stored on stack
    let arr = [1, 2, 3]; // Fixed-size array stored on stack
    
    // When this scope ends, x and arr are automatically deallocated
    */


    /*
    Heap Memory

    // Heap memory is memory that is manually managed by the program
    // - Memory must be explicitly allocated and deallocated
    // - Size can be dynamic and unknown at compile time
    // - Slower access than stack memory
    // - Only limited by system memory
    // - More flexible than stack memory
    
    // Example of heap allocation:
    let mut v = Vec::new(); // Vector stored on heap
    v.push(1); // Dynamically growing
    v.push(2);
    
    // Box type for heap allocation of single values
    let heap_int = Box::new(5); // Integer stored on heap
    
    // When v and heap_int go out of scope, their heap memory is freed
    // This is handled automatically in Rust but must be manual in some languages
    
     */

}


