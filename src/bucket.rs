use parquet::arrow::{ArrowReader, ParquetFileArrowReader};
use parquet::file::reader::{FileReader, SerializedFileReader};

pub fn split_parquet(pq_path: &str, batch_size: usize) {
    let start_time = Instant::now();

    let file = File::open(pq_path).unwrap();
    let file_reader = SerializedFileReader::new(file).unwrap();
    let mut arrow_reader = ParquetFileArrowReader::new(file_reader);

    let mut i = 0;
    for maybe_batch in arrow_reader.get_record_reader(batch_size).unwrap() {
        let batch = maybe_batch.unwrap();
        let column = batch.column(0);

        for row in 0..column.len() {
            if let Some(binary) = column.get_data::<parquet::data_type::ByteArray>(row) {
                let mut file = File::create(format!("{i}.mp4")).unwrap();
                file.write_all(binary.data()).unwrap();
            }
        }

        i += 1;
    }

    let end_time = Instant::now();
    println!("Time taken to split parquet file: {:?}", end_time - start_time);
}

pub fn sample(pq_path: &str, num_rows: usize) -> Result<Vec<Vec<String>>, parquet::errors::ParquetError> {
    let file = File::open(pq_path)?;
    let file_reader = SerializedFileReader::new(file)?;
    let mut arrow_reader = ParquetFileArrowReader::new(file_reader);

    let batch_size = num_rows.min(1024); // Read in chunks, but not exceeding desired num_rows
    let mut record_reader = arrow_reader.get_record_reader(batch_size)?;

    let mut samples = Vec::new();
    let schema = arrow_reader.get_schema()?;

    // Initialize sample vectors for each column
    for _ in 0..schema.fields().len() {
        samples.push(Vec::with_capacity(num_rows));
    }

    let mut rows_read = 0;
    while let Some(batch) = record_reader.next() {
        let batch = batch?;
        for col_idx in 0..batch.num_columns() {
            let column = batch.column(col_idx);

            for row_idx in 0..column.len().min(num_rows - rows_read) {
                let value = match column.get_data().as_any().downcast_ref::<arrow2::array::Utf8Array<i32>>() {
                    Some(array) => array.value(row_idx).to_string(),
                    _ => "null".to_string(), // Handle other data types as needed
                };
                samples[col_idx].push(value);
            }
        }

        rows_read += batch.num_rows();
        if rows_read >= num_rows {
            break;
        }
    }

    Ok(samples)
}