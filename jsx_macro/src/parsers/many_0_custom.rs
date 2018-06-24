macro_rules! many_0_custom(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use ::nom::lib::std::result::Result::*;
      // use ::nom::{Err,AtEof};
      use ::nom::Err;

      let ret;
      let mut res   = ::nom::lib::std::vec::Vec::new();
      let mut input = $i.clone();

      loop {
        let input_ = input.clone();
        match $submac!(input_, $($args)*) {
          Ok((i, o))              => {
            // N.B. removed by RB, presumably this needs to be replaced.
            // When I encounter an infinite-compile-time configuration, I'll fix this.

            // loop trip must always consume (otherwise infinite loops)
            // if i == input {

            //   if i.at_eof() {
            //     ret = Ok((input, res));
            //   } else {
            //     ret = Err(Err::Error(error_position!(input, ::nom::ErrorKind::Many0)));
            //   }
            //   break;
            // }
            res.push(o);

            input = i;
          },
          Err(Err::Error(_))      => {
            ret = Ok((input, res));
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
