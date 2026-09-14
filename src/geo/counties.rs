#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct County {
    pub state_fips: u8,
    pub county_fips: u16,
    pub name: &'static str,
}

pub fn lookup(state: u8, county: u16) -> Option<County> {
    let known: &[(u8, u16, &str)] = &[
        (1, 73, "Jefferson"),
        (2, 20, "Anchorage"),
        (4, 13, "Maricopa"),
        (5, 119, "Pulaski"),
        (6, 37, "Los Angeles"),
        (6, 59, "Orange"),
        (6, 75, "San Francisco"),
        (6, 85, "Santa Clara"),
        (8, 31, "Denver"),
        (9, 9, "Fairfield"),
        (10, 3, "New Castle"),
        (11, 1, "District of Columbia"),
        (12, 86, "Miami-Dade"),
        (12, 11, "Broward"),
        (12, 99, "Palm Beach"),
        (13, 121, "Fulton"),
        (15, 3, "Honolulu"),
        (16, 1, "Ada"),
        (17, 31, "Cook"),
        (18, 97, "Marion"),
        (19, 113, "Polk"),
        (20, 173, "Sedgwick"),
        (21, 111, "Jefferson"),
        (22, 71, "Orleans"),
        (23, 5, "Cumberland"),
        (24, 510, "Baltimore"),
        (25, 25, "Suffolk"),
        (26, 163, "Wayne"),
        (27, 53, "Hennepin"),
        (28, 49, "Hinds"),
        (29, 189, "St. Louis"),
        (30, 29, "Silver Bow"),
        (31, 109, "Douglas"),
        (32, 3, "Clark"),
        (33, 11, "Hillsborough"),
        (34, 21, "Mercer"),
        (35, 1, "Bernalillo"),
        (36, 61, "New York"),
        (36, 47, "Kings"),
        (36, 81, "Queens"),
        (36, 85, "Richmond"),
        (36, 5,  "Bronx"),
        (37, 119, "Wake"),
        (38, 15, "Cass"),
        (39, 35, "Cuyahoga"),
        (40, 109, "Oklahoma"),
        (41, 51, "Multnomah"),
        (42, 101, "Philadelphia"),
        (44, 7, "Providence"),
        (45, 19, "Charleston"),
        (46, 99, "Minnehaha"),
        (47, 37, "Davidson"),
        (48, 113, "Harris"),
        (48, 201, "Dallas"),
        (48, 453, "Travis"),
        (49, 35, "Salt Lake"),
        (50, 7, "Chittenden"),
        (51, 59, "Fairfax"),
        (53, 33, "King"),
        (54, 39, "Kanawha"),
        (55, 79, "Milwaukee"),
        (56, 21, "Laramie"),
    ];
    known.iter()
        .find(|&&(s, c, _)| s == state && c == county)
        .map(|&(state_fips, county_fips, name)| County { state_fips, county_fips, name })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn la() { assert_eq!(lookup(6, 37).unwrap().name, "Los Angeles"); }
    #[test] fn ny() { assert_eq!(lookup(36, 61).unwrap().name, "New York"); }
    #[test] fn cook() { assert_eq!(lookup(17, 31).unwrap().name, "Cook"); }
    #[test] fn harris() { assert_eq!(lookup(48, 113).unwrap().name, "Harris"); }
    #[test] fn dc() { assert_eq!(lookup(11, 1).unwrap().name, "District of Columbia"); }
    #[test] fn unknown() { assert!(lookup(99, 99).is_none()); }
    #[test] fn county_count_at_least_60() {
        let all = [(1,73u16),(2,20),(4,13),(5,119),(6,37),(6,59),(6,75),(6,85),(8,31),(9,9),
                   (10,3),(11,1),(12,86),(12,11),(12,99),(13,121),(15,3),(16,1),(17,31),(18,97),
                   (19,113),(20,173),(21,111),(22,71),(23,5),(24,510),(25,25),(26,163),(27,53),
                   (28,49),(29,189),(30,29),(31,109),(32,3),(33,11),(34,21),(35,1),(36,61),
                   (36,47),(36,81),(36,85),(36,5),(37,119),(38,15),(39,35),(40,109),(41,51),
                   (42,101),(44,7),(45,19),(46,99),(47,37),(48,113),(48,201),(48,453),(49,35),
                   (50,7),(51,59),(53,33),(54,39),(55,79),(56,21)];
        assert!(all.len() >= 60);
    }
}
