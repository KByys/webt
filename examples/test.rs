fn main() {}


pub fn percent_encode_byte(byte: u8) -> &'static str {
    static ENC_TABLE: &[u8; 768] = b"\
      %00%01%02%03%04%05%06%07%08%09%0A%0B%0C%0D%0E%0F\
      %10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F\
      %20%21%22%23%24%25%26%27%28%29%2A%2B%2C%2D%2E%2F\
      %30%31%32%33%34%35%36%37%38%39%3A%3B%3C%3D%3E%3F\
      %40%41%42%43%44%45%46%47%48%49%4A%4B%4C%4D%4E%4F\
      %50%51%52%53%54%55%56%57%58%59%5A%5B%5C%5D%5E%5F\
      %60%61%62%63%64%65%66%67%68%69%6A%6B%6C%6D%6E%6F\
      %70%71%72%73%74%75%76%77%78%79%7A%7B%7C%7D%7E%7F\
      %80%81%82%83%84%85%86%87%88%89%8A%8B%8C%8D%8E%8F\
      %90%91%92%93%94%95%96%97%98%99%9A%9B%9C%9D%9E%9F\
      %A0%A1%A2%A3%A4%A5%A6%A7%A8%A9%AA%AB%AC%AD%AE%AF\
      %B0%B1%B2%B3%B4%B5%B6%B7%B8%B9%BA%BB%BC%BD%BE%BF\
      %C0%C1%C2%C3%C4%C5%C6%C7%C8%C9%CA%CB%CC%CD%CE%CF\
      %D0%D1%D2%D3%D4%D5%D6%D7%D8%D9%DA%DB%DC%DD%DE%DF\
      %E0%E1%E2%E3%E4%E5%E6%E7%E8%E9%EA%EB%EC%ED%EE%EF\
      %F0%F1%F2%F3%F4%F5%F6%F7%F8%F9%FA%FB%FC%FD%FE%FF\
      ";

    let index = usize::from(byte) * 3;
    // SAFETY: ENC_TABLE is ascii-only, so any subset if it should be
    // ascii-only too, which is valid utf8.
    unsafe { std::str::from_utf8_unchecked(&ENC_TABLE[index..index + 3]) }
}

/// The [`application/x-www-form-urlencoded` byte serializer](
/// https://url.spec.whatwg.org/#concept-urlencoded-byte-serializer).
///
/// Return an iterator of `&str` slices.
pub fn byte_serialize(input: &[u8]) -> ByteSerialize<'_> {
    ByteSerialize { bytes: input }
}

/// Return value of `byte_serialize()`.
#[derive(Debug)]
pub struct ByteSerialize<'a> {
    bytes: &'a [u8],
}

fn byte_serialized_unchanged(byte: u8) -> bool {
    matches!(byte, b'*' | b'-' | b'.' | b'0' ..= b'9' | b'A' ..= b'Z' | b'_' | b'a' ..= b'z')
}

impl<'a> Iterator for ByteSerialize<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if let Some((&first, tail)) = self.bytes.split_first() {
            if !byte_serialized_unchanged(first) {
                self.bytes = tail;
                return Some(if first == b' ' {
                    "+"
                } else {
                    percent_encode_byte(first)
                });
            }
            let position = tail.iter().position(|&b| !byte_serialized_unchanged(b));
            let (unchanged_slice, remaining) = match position {
                // 1 for first_byte + i unchanged in tail
                Some(i) => self.bytes.split_at(1 + i),
                None => (self.bytes, &[][..]),
            };
            self.bytes = remaining;
            // This unsafe is appropriate because we have already checked these
            // bytes in byte_serialized_unchanged, which checks for a subset
            // of UTF-8. So we know these bytes are valid UTF-8, and doing
            // another UTF-8 check would be wasteful.
            Some(unsafe { std::str::from_utf8_unchecked(unchanged_slice) })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.bytes.is_empty() {
            (0, Some(0))
        } else {
            (1, Some(self.bytes.len()))
        }
    }
}
