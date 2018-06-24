macro_rules! many_0_custom(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use ::nom::lib::std::result::Result::*;
      // use ::nom::{Err,AtEof};
      use ::nom::Err;

      // ret is Ok|Err
      let ret;
      let mut vec_of_responses = ::nom::lib::std::vec::Vec::new();
      let mut input = $i.clone();

      loop {
        let input_ = input.clone();
        match $submac!(input_, $($args)*) {
          Ok((i, o))              => {
            // i is remaining
            // o is matched

            // N.B. I don't know if this is actually solves the infinite loops...

            // loop trip must always consume (otherwise infinite loops)
            if i.len() == 0 || i.len() == input.len() {
              vec_of_responses.push(o);
              ret = Ok((input, vec_of_responses));
              break;
            }
            // if i == input {
            //   if i.at_eof() {
            //     ret = Ok((input, res));
            //   } else {
            //     ret = Err(Err::Error(error_position!(input, ::nom::ErrorKind::Many0)));
            //   }
            //   break;
            // }
            vec_of_responses.push(o);

            input = i;
          },
          Err(Err::Error(_))      => {
            ret = Ok((input, vec_of_responses));
            break;
          },
          Err(e) => {
            ret = Err(e);
            break;
          },
        }
      }

      ret
    }
  );
  ($i:expr, $f:expr) => (
    many_0_custom!($i, call!($f));
  );
);
