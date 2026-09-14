# ISO 19109 Application Schema

Rules for the tmig application schema:
1. Every feature type has a name and a geometry type.
2. Every attribute has a domain and a cardinality.
3. Every relation has source and target feature types.
4. Every identifier is unique within its namespace.

Feature types:
  Zip5:          attribute id: u32, geometry: point
  AreaCode:      attribute code: u16, geometry: polygon
  Boundary11:    attribute id: [u8; 11], geometry: polygon
  County:        attribute fips: (u8, u16), geometry: polygon
  State:         attribute fips: u8, geometry: polygon
