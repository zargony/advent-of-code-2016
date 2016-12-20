use std::collections::HashMap;
use std::str::FromStr;


/// A room, described by its encrypted name
#[derive(Debug, PartialEq, Eq)]
pub struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Room, &'static str> {
        s.rfind('[').and_then(|ic| {
            let checksum = s[ic+1..s.len()-1].to_owned();
            s[..ic].rfind('-').and_then(|is| {
                let name = s[..is].to_owned();
                s[is+1..ic].parse().map(|sector|
                    Room {
                        encrypted_name: name,
                        sector_id: sector,
                        checksum: checksum,
                    }
                ).ok()
            })
        }).ok_or("Invalid room format")
    }
}

impl Room {
    /// Parse a text with room descriptions
    fn parse(s: &str) -> Result<Vec<Room>, &'static str> {
        s.lines().map(|line| line.parse()).collect()
    }

    /// Calculate checksum
    fn calculate_checksum(&self) -> String {
        let mut charcounts = HashMap::new();
        for ch in self.encrypted_name.bytes() {
            if ch != b'-' {
                *charcounts.entry(ch).or_insert(0) += 1;
            }
        }
        let mut chars: Vec<_> = charcounts.keys().map(|ch| *ch).collect();
        chars.sort_by(|a, b| {
            let ac = charcounts.get(a).unwrap();
            let bc = charcounts.get(b).unwrap();
            if ac == bc { a.cmp(b) } else { bc.cmp(ac) }
        });
        chars.truncate(5);
        String::from_utf8(chars).unwrap()
    }

    /// Checks if the room is real (i.e. its checksum is valid)
    fn is_real(&self) -> bool {
        self.calculate_checksum() == self.checksum
    }

    /// Decrypt room name
    fn name(&self) -> Result<String, &'static str> {
        self.encrypted_name.bytes().map(|ch| {
            let ofs = (self.sector_id % 26) as u8;
            match ch {
                b'a'...b'z' => Ok((((ch - b'a' + ofs) % 26) + b'a') as char),
                b'-' => Ok(' '),
                _ => Err("Unsupported encryption format"),
            }
        }).collect()
    }
}


fn main() {
    let rooms = Room::parse(include_str!("day04.txt")).unwrap();
    let sector_sum = rooms.iter().filter(|room| room.is_real()).fold(0, |sum, room| sum + room.sector_id);
    println!("Sum of sector ids of real rooms: {}", sector_sum);
    let sector_id = rooms.iter().find(|room| room.name().unwrap() == "northpole object storage").unwrap().sector_id;
    println!("North pole object storage sector id: {}", sector_id);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let rooms = Room::parse("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]").unwrap();
        assert_eq!(rooms[0], Room { encrypted_name: "aaaaa-bbb-z-y-x".to_owned(), sector_id: 123, checksum: "abxyz".to_owned() });
        assert_eq!(rooms[1], Room { encrypted_name: "a-b-c-d-e-f-g-h".to_owned(), sector_id: 987, checksum: "abcde".to_owned() });
        assert_eq!(rooms[2], Room { encrypted_name: "not-a-real-room".to_owned(), sector_id: 404, checksum: "oarel".to_owned() });
        assert_eq!(rooms[3], Room { encrypted_name: "totally-real-room".to_owned(), sector_id: 200, checksum: "decoy".to_owned() });
    }

    #[test]
    fn checking() {
        let rooms = Room::parse("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]").unwrap();
        assert!( rooms[0].is_real());
        assert!( rooms[1].is_real());
        assert!( rooms[2].is_real());
        assert!(!rooms[3].is_real());
    }

    #[test]
    fn decrypting() {
        let rooms = Room::parse("qzmt-zixmtkozy-ivhz-343[]").unwrap();
        assert_eq!(rooms[0].name(), Ok("very encrypted name".to_owned()));
    }
}
