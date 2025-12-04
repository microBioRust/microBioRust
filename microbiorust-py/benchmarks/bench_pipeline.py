import os
import microbiorust
import time

class PipelineSuite:
    """
    Benchmarks for microbiorust-py

    This suite measures both time and memory performance across
    different parsing operations to track optimization improvements.
    """

    timeout = 300  # 5 minute timeout per benchmark

    # Setup runs before the timer starts
    def setup(self):
        bench_dir = os.path.dirname(__file__)

        # Join it with the test filename
        self.filepath = os.path.join(bench_dir, "Rhiz3841.gbk.gb")

        # Check if it exists just to be safe (helps debugging)
        if not os.path.exists(self.filepath):
            raise FileNotFoundError(f"Could not find benchmark file at: {self.filepath}")

    # === PRIMARY BENCHMARKS ===

    # 1. Benchmark TIME of protein count function (main metric)
    def time_process_all(self):
        """
        Primary benchmark: Measures parsing speed for GenBank to protein FASTA conversion.
        This is the key metric that should improve 20-40% with regex optimizations.
        """
        _ = microbiorust.gbk_to_faa_count(self.filepath)

    # 2. Benchmark MEMORY of protein count function (main metric)
    def peakmem_process_all(self):
        """
        Primary benchmark: Measures peak memory usage during parsing.
        Should remain stable or improve slightly with buffer pre-allocation.
        """
        _ = microbiorust.gbk_to_faa_count(self.filepath)

    # === DETAILED BENCHMARKS ===

    # 3. Benchmark parsing throughput
    def time_process_all_iterations(self):
        """
        Measures sustained parsing performance over 5 iterations.
        Tests regex caching effectiveness and memory stability.
        """
        for _ in range(5):
            _ = microbiorust.gbk_to_faa_count(self.filepath)

    # 4. Track parsing latency (single pass)
    def track_parsing_latency(self):
        """
        Track metric: Records single-pass parsing latency.
        Useful for trending analysis over time.
        """
        start = time.perf_counter()
        _ = microbiorust.gbk_to_faa_count(self.filepath)
        end = time.perf_counter()
        return end - start

