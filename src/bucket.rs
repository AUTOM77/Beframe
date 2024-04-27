use polars::prelude::*;

pub fn sample(pq_path: &str, num_rows: usize) -> Result<(), Box<dyn std::error::Error>> {

    let df = LazyFrame::scan_parquet(pq_path, ScanArgsParquet::default())?
        .select([all(),])
        .collect()?;
    println!("{}", df);

    Ok(())
}