fn main() {
    
    // For functions that need to take a collection of objects, slices are usually a good choice:

    fn work_on_bytes(slice: &[u8]) {}
    // Because Vec<T> and arrays [T; N] implement Deref<Target=[T]>, they can be easily coerced to a slice:
    
    let vec = Vec::new();
    work_on_bytes(&vec);
    
    let arr = [0; 10];
    work_on_bytes(&arr);
    
    let slice = &[1,2,3];
    work_on_bytes(slice); // Note lack of &, since it doesn't need coercing
    // However, instead of explicitly requiring a slice, the function can be made to accept any type that can be used as a slice:
    
    fn work_on_bytes<T: AsRef<[u8]>>(input: T) {
        let slice = input.as_ref();
    }
    // In this example the function work_on_bytes will take any type T that implements as_ref(), which returns a reference to [u8].
    
    // work_on_bytes(vec);
    // work_on_bytes(arr);
    work_on_bytes(slice);
    work_on_bytes("strings work too!");

}
