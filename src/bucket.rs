use std::io::Write;
use polars::prelude::*;
use rayon::prelude::*;

pub fn sample(pq_path: &str, batch_size: usize) -> Result<(), PolarsError> {
    let df: DataFrame = LazyFrame::scan_parquet(pq_path, Default::default())?
        .select([col("video")])
        .collect()?;

    let video_series = df.column("video")?.binary()?;

    video_series.iter().enumerate().into_iter()
        .par_bridge().for_each(|(i, video)| {
            if let Some(video_data) = video {
                let name = format!("{:04}.mp4", i);
                let mut output_file = std::fs::File::create(name).unwrap();
                let _ = output_file.write_all(video_data);
            }
        });
    Ok(())
}
