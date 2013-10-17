// switch - case

fn main(){ 
let test = 3;
match test {
  0     => println("zero"),
  1 | 2 => println("one or two"),
  3..10 => println("three to ten"),
  _     => println("something else")
}

}
