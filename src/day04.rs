use std::collections::HashMap;
use std::str::FromStr;

/// A room, described by its encrypted name
#[derive(Debug, PartialEq, Eq)]
pub struct Room<'a> {
    encrypted_name: &'a str,
    sector_id: u32,
    checksum: &'a str,
}

impl<'a> From<&'a str> for Room<'a> {
    fn from(s: &str) -> Room {
        let ic = s.rfind('[').unwrap();
        let checksum = &s[ic+1..s.len()-1];
        let is = s[..ic].rfind('-').unwrap();
        let sector = u32::from_str(&s[is+1..ic]).unwrap();
        let name = &s[..is];
        Room {
            encrypted_name: name,
            sector_id: sector,
            checksum: checksum,
        }
    }
}

impl<'a> Room<'a> {
    /// Parse a text with room descriptions. Panics on parsing errors.
    fn parse(s: &str) -> Vec<Room> {
        s.lines().map(|line| {
            Room::from(line)
        }).collect()
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
}

fn main() {
    let rooms = Room::parse(include_str!("day04.txt"));
    let sector_sum = rooms.iter().fold(0, |sum, room| sum + if room.is_real() { room.sector_id } else { 0 });
    println!("Sum of sector ids of real rooms: {}", sector_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let rooms = Room::parse("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]");
        assert_eq!(rooms[0], Room { encrypted_name: "aaaaa-bbb-z-y-x", sector_id: 123, checksum: "abxyz" });
        assert_eq!(rooms[1], Room { encrypted_name: "a-b-c-d-e-f-g-h", sector_id: 987, checksum: "abcde" });
        assert_eq!(rooms[2], Room { encrypted_name: "not-a-real-room", sector_id: 404, checksum: "oarel" });
        assert_eq!(rooms[3], Room { encrypted_name: "totally-real-room", sector_id: 200, checksum: "decoy" });
    }

    #[test]
    fn checking() {
        let rooms = Room::parse("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]");
        assert!( rooms[0].is_real());
        assert!( rooms[1].is_real());
        assert!( rooms[2].is_real());
        assert!(!rooms[3].is_real());
    }
}
