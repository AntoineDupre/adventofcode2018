// My original plan:
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
// There is collisition when a cell is above than one


use std::io::Result;
use std::io;
use std::fs;
use std::collections::HashMap;


struct Claim{
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Claim{
    fn new(word: &str) -> Claim {
        // TODO: This does not look very rusty
        // word like #1272 @ 873,39: 24x26
        let mut iter_body = word.split("@");
        // get #1272 and remove #
        let _id: String = iter_body.next().unwrap().trim().chars().skip(1).collect();
        // convert to int
        let id = _id.parse::<i32>().unwrap();
        // Now parse the body, split between origin position and size
        let mut iter_claim = iter_body.next().unwrap().trim().split(":");
        // x and y are separated by ","
        let mut iter_point = iter_claim.next().unwrap().trim().split(",");
        // width and height are separated by "x"
        let mut iter_size = iter_claim.next().unwrap().trim().split("x");
        // Now lets convert everything to int
        let x = iter_point.next().unwrap().trim().parse::<i32>().unwrap();
        let y = iter_point.next().unwrap().trim().parse::<i32>().unwrap();
        let width = iter_size.next().unwrap().trim().parse::<i32>().unwrap();
        let height = iter_size.next().unwrap().trim().parse::<i32>().unwrap();
        // Create the claim
        Claim { id, x, y, width, height }
    }


    // Note: https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md
    fn iterate_cell(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        // Return an iterator that contains all the cells coordinates orccupied by the claim
        // Note: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
        (self.x..self.x+self.width).flat_map(move |x| (self.y..self.y+self.height).map(move |y| (x, y, self.id)))
    }
}


struct Fabric{
    used_squares: HashMap<(i32, i32), i32>,
    claims_map: HashMap<(i32, i32), Vec<i32>>
}

impl Fabric{
    fn new(f: &str) -> Fabric{
        let mut used_squares = HashMap::new();
        let mut claims_map = HashMap::new();
        f.lines()
            .map(|word| Claim::new(word))
            .for_each(|c| {
                c.iterate_cell().for_each(|(x, y, id)|{
                    // For each cell used by this claim, increment the
                    // factory<x,y> cell count.
                    let count = used_squares.entry((x, y)).or_insert(0);
                    *count += 1;
                    //let c = claims_colisions.entry(id).or_insert(0);
                    //if *count > 2 {
                    //    *c += 1
                    //}

                    let claim = claims_map.entry((x, y)).or_insert(Vec::new());
                    claim.push(id);
                })
            });
        Fabric { used_squares, claims_map}
}
    fn collision_count(&self) -> i32{
        let mut duplicates = 0;
        for v in self.used_squares.values(){
            if *v > 1 {
                duplicates += 1;
            }
        }
        return duplicates
    }
    fn no_collisition_id(&self) {
        //let mut output = Vec::new();
        for (k, v) in self.claims_map.iter()
            .filter(|&(k, v)| v.len() < 2){
                //output.push(*v);
                println!("{:?}", v);
            }
    }
}
// split dans deux ficheir

fn main() {
    // Read input file
    let f =  fs::read_to_string("input/input").expect("Cannot read file");
    // Build a fabric
    let fabric  = Fabric::new(&f);
    // Display collistions.
    println!("collision {} {:?}", fabric.collision_count(), fabric.no_collisition_id());
}
