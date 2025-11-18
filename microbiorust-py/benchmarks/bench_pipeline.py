import os
import microbiorust
import time

class PipelineSuite:
    """
    Benchmarks for microbiorust-py
    """
    # Setup runs before the timer starts
    def setup(self):
        bench_dir = os.path.dirname(__file__)
        
        # 2. Join it with the test filename
        self.filepath = os.path.join(bench_dir, "Rhiz3841.gbk.gb")

        # Check if it exists just to be safe (helps debugging)
        if not os.path.exists(self.filepath):
            raise FileNotFoundError(f"Could not find benchmark file at: {self.filepath}")
    # 1. Benchmark TIME of protein count function
    def time_process_all(self):
        # This calls the microBioRust function that returns a count of protein fasta per file
     
        _ = microbiorust.gbk_to_faa_count(self.filepath)
    # 2. Benchmark MEMORY of protein count function
    def peakmem_process_all(self):
        
        _ = microbiorust.gbk_to_faa_count(self.filepath)

