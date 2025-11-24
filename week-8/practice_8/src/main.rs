 fn main() {
      // initialize a mutable tuple
      let mut mouintain_heights = ("Everest", 8848, "Fishtail", 6993);

      println!("Original tuple = n{:?}", mouintain_heights);

      // change 3rd and 4th element of a mutable tuple
      mouintain_heights.2 = "Lhotse";
      mouintain_heights.3 = 8516;

      println!("changed tuple = {:?}", mouintain_heights);
}