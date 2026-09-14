# Spectral Cross-Cutting Diagnostics

A spectral diagnostic computes a signal decomposition across the corpus.
The signal is any measurable axis. The decomposition reveals which axes
separate sources, which sources cluster, and which are outliers.

## Axes

  A1  authority class (C1..C8)
  A2  license type (public domain, CC0, CC-BY, ODbL, other)
  A3  size decade (small, KB, MB, GB, TB, PB)
  A4  geographic scope (US federal, US state, international, municipal)
  A5  data type (tabular, text, image, geospatial, time series, graph)
  A6  update cadence (real-time, daily, monthly, annual, static)
  A7  access mode (bulk download, API, HTML scrape, FTP)
  A8  domain (economics, health, environment, security, standards, science)

## Cross-cuts

Any pair of axes (Ai, Aj) forms a 2-D grid. The count in cell (v_i, v_j)
is a cross-cut. The matrix of cross-cuts is the spectral signature of
the corpus.

## Diagnostic output

  docs/engagements/*.md        engagement dossiers
  docs/external/*.md           retrieval records
  corpus/provenance/*.csv      source list
  corpus/provenance/summary.md counts per class
  corpus/spectral/*.tsv        cross-cut matrices
