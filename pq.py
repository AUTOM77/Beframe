import pyarrow.parquet as pq
import time

def split_parquet(pq_path, batch_size=1):
    start_time = time.time()
    i = 0
    pf = pq.ParquetFile(pq_path)
    for batch in pf.iter_batches(batch_size):
        df = batch.to_pandas()
        for binary in df["video"]:
            if(binary):
                with open(f"{i}.mp4", "wb") as f:
                    _ = f.write(binary)
        del df
        i += 1
    end_time = time.time()
    print(f"Time taken to split parquet file: {end_time - start_time} seconds")