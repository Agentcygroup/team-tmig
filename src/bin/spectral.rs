use tmig::corpus::{Axis, Manifest, Spectral};
use tmig::spectral::eig::power_iteration;
use tmig::spectral::entropy::shannon_entropy;
use tmig::spectral::matrix::Matrix;
use tmig::spectral::rank::rank;

fn main() -> std::io::Result<()> {
    let m = Manifest::load("corpus/provenance/manifest.csv")?;
    println!("manifest\t{}", m.len());
    let mut total_rank = 0;
    let mut total_entropy = 0.0;
    let mut count = 0;
    for a in Axis::all() {
        for b in Axis::all() {
            let s = Spectral::compute(&m, *a, *b);
            let mat = Matrix::from_map(&s.matrix);
            let r = rank(&mat, 1e-9);
            let rs = mat.row_sums();
            let h = shannon_entropy(&rs);
            let (_, l) = if mat.rows() == mat.cols() {
                power_iteration(&mat.data, 50)
            } else {
                (vec![], 0.0)
            };
            println!(
                "{}\t{}\t{}x{}\tr={}\tH={:.4}\tlam={:.4}",
                a.name(),
                b.name(),
                mat.rows(),
                mat.cols(),
                r,
                h,
                l
            );
            total_rank += r;
            total_entropy += h;
            count += 1;
        }
    }
    println!(
        "summary\tsum_rank={}\tsum_entropy={:.4}\tcount={}",
        total_rank, total_entropy, count
    );
    Ok(())
}
