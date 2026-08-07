test_that("r_amino_counts returns correct data frame", {
  #we know this exact 20-aa sequence should have exactly 1 of each
  seq <- "ACDEFGHIKLMNPQRSTVWY"
  res <- r_amino_counts(seq)
  
  #check the structure
  expect_s3_class(res, "data.frame")
  expect_equal(nrow(res), 20)
  expect_equal(colnames(res), c("AminoAcid", "Count"))
  
  #check the maths (all counts should be 1)
  expect_true(all(res$Count == 1))
  
  #check a specific lookup
  expect_equal(res$Count[res$AminoAcid == "A"], 1)
})

test_that("r_amino_percentage calculates correct frequencies", {
  #20 amino acids, each appears once, so each should be exactly 5%
  seq <- "ACDEFGHIKLMNPQRSTVWY"
  res <- r_amino_percentage(seq)
  
  expect_s3_class(res, "data.frame")
  expect_equal(colnames(res), c("AminoAcid", "Percentage"))
  expect_true(all(res$Percentage == 5.0))
  
  #testing an unbalanced sequence
  #"AAAAAC" -> length 6. A = 5/6 (83.33%), C = 1/6 (16.66%)
  res_unbalanced <- r_amino_percentage("AAAAAC")
  a_pct <- res_unbalanced$Percentage[res_unbalanced$AminoAcid == "A"]
  expect_equal(a_pct, (5/6) * 100, tolerance = 1e-4)
})

test_that("r_aromaticity returns correct relative frequencies", {
  #aromatic acids are Y, W, F. 
  #if a sequence is ONLY these, aromaticity is 1.0 (100%)
  expect_equal(r_aromaticity("YWF"), 1.0)
  expect_equal(r_aromaticity("YYYY"), 1.0)
  
  #if a sequence has none of these, it should be 0.0
  expect_equal(r_aromaticity("ACDEG"), 0.0)
  
  #mixed sequence: 2 aromatics out of 4 total = 0.5
  expect_equal(r_aromaticity("YWAC"), 0.5)
})

test_that("r_instability_index calculates the index without crashing", {
  #since the exact sum depends on the published specific CSV weights, 
  #ee start by ensuring it returns a single numeric value safely.
  seq <- "ACDEFGHIKLMNPQRSTVWY"
  res <- r_instability_index(seq)
  
  expect_type(res, "double")
  expect_length(res, 1)
  
  #if you know the exact expected index for a specific sequence based on your CSV, 
  #you can hardcode the expected value like this:
  #expect_equal(r_instability_index("MYSPECIFICSEQ"), 42.5, tolerance = 1e-4)
})
