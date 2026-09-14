# TIGER/Line National Retrieval

Authority: United States Census Bureau
Host: www2.census.gov
Directory: /geo/tiger/TIGER2023/TRACT/
File pattern: tl_2023_<FIPS>_tract.zip

State FIPS codes to fetch (52 files):
  01 02 04 05 06 08 09 10 11 12 13 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 44 45 46 47 48 49 50 51 53 54 55 56 60 66 69 72 78

File sizes: 1–40 MB each
Total compressed: roughly 600 MB
Total uncompressed: roughly 2 GB

One-shot command:
  for s in 01 02 04 05 06 08 09 10 11 12 13 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 44 45 46 47 48 49 50 51 53 54 55 56 60 66 69 72 78; do
    curl -O "https://www2.census.gov/geo/tiger/TIGER2023/TRACT/tl_2023_${s}_tract.zip"
  done
