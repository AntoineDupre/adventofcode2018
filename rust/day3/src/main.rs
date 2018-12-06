// Ideas:
// Lets parse the input and build some claims
// A claim shall have
//  - ID
//  - Recangle
//    - x
//    - y
//    - width
//    - height
// Maybe, in order to feed the factory
// a claim can return a iterable that contains for each point (x, y)
// A factory is a HashMap<x, y>.
// for claim in claims
//   for point in claims
//      factory[(x, y)] += 1
// There is collisition when a cell is above than one.


struct Claim{
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Claim{
    // https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md
    fn iterate_cell(&self) -> impl Iterator<Item = (i32, i32)>  {
        // we can define in rust an iterator like "let iter = 0..4;"
        // let iter_y = self.y..self.y + self.height;
        // let iter_x = self.x..self.x + self.width;
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
        (self.x..self.x+self.width).flat_map(|x| (self.y..self.y+self.height).map(|y| (x, y)))
        // Not working :
    }
}



fn main() {
    // let build a fabric
    println!("Hello, world!");
}
