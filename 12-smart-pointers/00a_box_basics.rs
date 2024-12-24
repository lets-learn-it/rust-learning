
fn main() {
    let b: Box<(i32, &str)>;
    {
        let t = (12, "eggs");  // will be dropped
        let c = Box::new(t);   // will be dropped but data on heap will stay
        b = c;                 // keep Box
    }
    println!("{:?}", b);

    let t = (12, "eggs");  // created on the stack
    let b = Box::new(t);   // created on the heap, but b was stored on stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y);
}
