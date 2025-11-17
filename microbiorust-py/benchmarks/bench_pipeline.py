import os
import microbiorust

class PipelineSuite:
    """
    Benchmarks for microbiorust-py
    """
    # Setup asv parameters - suggested on ArcticDB website
    rounds = 1
    number = 1
    repeat = 2
    min_run_count = 1
    warmup_time = 0
    # Setup runs before the timer starts
    def setup(self):
        bench_dir = os.path.dirname(__file__)
        
        # 2. Join it with your filename
        self.filepath = os.path.join(bench_dir, "Rhiz3841.gbk.gb")

        # Check if it exists just to be safe (helps debugging)
        if not os.path.exists(self.filepath):
            raise FileNotFoundError(f"Could not find benchmark file at: {self.filepath}")
    # 1. Benchmark TIME
    def time_process_all(self):
        # This calls your function that returns Vec<String>
       result = microbiorust.gbk_to_faa(self.filepath)
       for _ in result:
         pass

    # 2. Benchmark MEMORY (The known spike)
    def peakmem_process_all(self):
        result = microbiorust.gbk_to_faa(self.filepath)
        for _ in result:
          pass
