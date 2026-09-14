use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: Vec<String>,
    pub cols: Vec<String>,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn from_map(m: &BTreeMap<(String, String), usize>) -> Self {
        let mut rows: Vec<String> = m.keys().map(|(a, _)| a.clone()).collect();
        let mut cols: Vec<String> = m.keys().map(|(_, b)| b.clone()).collect();
        rows.sort();
        rows.dedup();
        cols.sort();
        cols.dedup();
        let ri: BTreeMap<_, _> = rows
            .iter()
            .enumerate()
            .map(|(i, r)| (r.clone(), i))
            .collect();
        let ci: BTreeMap<_, _> = cols
            .iter()
            .enumerate()
            .map(|(i, c)| (c.clone(), i))
            .collect();
        let mut d = vec![vec![0.0; cols.len()]; rows.len()];
        for ((a, b), v) in m {
            d[ri[a]][ci[b]] = *v as f64;
        }
        Self {
            rows,
            cols,
            data: d,
        }
    }
    pub fn shape(&self) -> (usize, usize) {
        (self.rows.len(), self.cols.len())
    }
    pub fn rows(&self) -> usize {
        self.rows.len()
    }
    pub fn cols(&self) -> usize {
        self.cols.len()
    }
    pub fn row_sums(&self) -> Vec<f64> {
        self.data.iter().map(|r| r.iter().sum()).collect()
    }
    pub fn sum(&self) -> f64 {
        self.data.iter().flat_map(|r| r.iter()).sum()
    }
}
