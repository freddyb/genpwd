extern crate rand;

fn main() {
        // password is at least this many chars long
        let minlength = 18;
        // up to 0 to 3 more, for fun
        let variance = rand::random::<(u8)>() & 0b11;
        let mut length = minlength + variance;

        while length > 0 {
          // filter high-ascii
          let c = rand::random::<u8>() & 127;
          if c > 32 { // skips control characters
            print!("{}", c as char);
            length -= 1;
          }
        }
        print!("\n");
}
