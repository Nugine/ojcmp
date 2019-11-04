use crate::chars::{CharsLike, EOF};
use crate::compare::compare;

use std::io::Cursor;
use std::io::Read;

impl CharsLike for Cursor<&[u8]> {
    fn next_char(&mut self) -> u16 {
        let mut buf = [0_u8; 1];
        match self.read_exact(&mut buf) {
            Ok(_) => u16::from(buf[0]),
            Err(_) => EOF,
        }
    }
    fn next_char_strip_cr(&mut self) -> u16 {
        let c = self.next_char();
        if c == EOF || (c as u8) != b'\r' {
            return c;
        }
        let n = self.next_char();
        if n == EOF {
            return u16::from(b'\r');
        }
        if (n as u8) == b'\n' {
            return u16::from(b'\n');
        }
        let pos = self.position() - 1;
        self.set_position(pos);
        u16::from(b'\r')
    }
}

#[cfg(test)]
macro_rules! judge {
    ($ret:expr, $std:expr,$user:expr) => {{
        let mut std = Cursor::new(&$std[..]);
        let mut user = Cursor::new(&$user[..]);

        let ret = compare(&mut std, &mut user);
        assert_eq!(ret, $ret);
    }};
}

#[test]
fn test_compare() {
    use crate::compare::Comparison::*;

    judge!(WA, b"1", b"2");
    judge!(WA, b"1\r\n", b"2\n");
    judge!(PE, b"1\r3\n", b"1\t3\n");
    judge!(PE, b"1 3\n", b"1\t3\n");
    judge!(PE, b"1 3\n", b"1         3\n");
    judge!(PE, b"1 3\r\n", b"1         3\r\n");
    judge!(PE, b"1 3\r\n", b"1         3\n");
    judge!(PE, b"1 3\n", b"1         3\r\n");
    judge!(PE, b"1\r3\t4\n", b"1\r3\r4\r\n");
    judge!(AC, b"1 2\n3 4", b"1 2\r\n3 4\n");
    judge!(AC, b"1 2 \n3 4", b"1 2 \r\n3 4 \n");
    judge!(AC, b"\n", b"");
    judge!(AC, b"", b"\n");
    judge!(AC, b" \n", b" ");
    judge!(AC, b"1\n", b"1");
    judge!(AC, b"1 \n", b"1");
    judge!(AC, b"1 \n", b"1\n");
    judge!(AC, b"1\t\n", b"1\r\n");
    judge!(AC, b"1\r\n", b"1\r");
    judge!(AC, b"1 2  \n3 4", b"1 2    \t\n3 4");
    judge!(AC, b"1 2 \r\n3 4", b"1 2                  \r\n3 4");
    judge!(AC, b"1\r\n\r\n\r\n", b"1  ");
}
