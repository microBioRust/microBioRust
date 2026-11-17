# Changelog #
---

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project should adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## [Unreleased] 17/11/26

### Added
- Ability to use a protein alignment MSA to create consensus sequence, previously there was only functionality for a DNA MSA, moved the function
  out of the Alignment<DNA> to generic and added an extra underpinning function to count frequencies
- Test for the protein alignment in microBioRust and microbiorust-py

## Changed
- microbiorust-py built against microBioRust 0.2.0 (`path` dependency) 
- `impl_build_features!`, `impl_build_sequences!`, `impl_build_faa!` and
  `impl_build_ffn!` read struct fields instead of iterating over enum variants, from legacy enum Attributes.
- `PyFeatureInfo.extras` now holds `transl_table` and `feature_type` - this may not be enough, extras may be widened
- R-CMD-check workflow is restricted to changes in `microbiorust-r/`, not to run on every push to any crate
- apply clippy and fmt suggestions to seqmetrics @modernmedici
- README: point the usage link at the hosted docs- @modernmedici
- microbiorust-py: opt out of the automatic FromPyObject derive - @modernmedici

### Fixed

- Removed a leftover enum-style filter on `SequenceAttributes` in microbiorust-py `lib.rs`.

## [Unreleased] 06/11/26

### Added
- A `FeatureKind` enum for GenBank/EMBL feature keys to hold the feature type (`gene`, `CDS`, `tRNA`, `rRNA`,
  `tmRNA`, `ncRNA`, `misc_RNA`, `regulatory`, `repeat_region`, `mobile_element`,
  `misc_feature`, other), with `from_key`, and `is_non_coding` to access the data
- Non-coding features (tRNA, rRNA, ncRNA and similar) are now parsed from GenBank and
  EMBL records into `Record::nc_features` (`NonCodingAttributes`, and
  `NonCodingAttributeBuilder`). Joined locations (for joined feature) are now stored as one entry per segment.
- `feature_type` attribute is now added on CDS features.
- `feature_key` and `embl_feature_key` helpers for detecting feature header lines, cleaner than before listing each 
  possibel option in turn
- Test added for joined feature locations

### Fixed
- Parser hang on records with no CDS after first implementing the non coding feature parsing.  
  The source qualifier loop only stopped at a CDS line, so a feature-less record could loop forever in a multi-record file,
  or run on into the next record.
- The CDS qualifier loop now does not read through a following non-coding feature, which
  could possibly overwrite the CDS's product or locus tag. It stops at any feature header.
- Records with a `CONTIG` line followed by `ORIGIN` are no longer rejected as master
  records, this type of output comes from PGAP annotation pipeline of NCBI.

### Changed
- Accessors on `Record` (`cds()`, `source_map()`, `non_coding()`, `seq_features()`)
  return references instead of clones

### Internal
- Clippy: removed the inherent `default()` from `create_builder!` (builders derive
  `Default`), derived `Default` for `FeatureKind`, replaced with `or_default()`, and 
   tidied `Option` handling in the sequence loop.

## Unreleased ##

### Added ###
- docs
- windows install description for docs
- this changelog

### Changed ###
- Ran cargo update and removed unused crates futures and thiserror
- Replaced AttributeBuilder<K, HashSet<V>> in record.rs with a single-value AttributeBuilder<K, V> 
  enforcing one entry per locus tag; duplicate keys now halt the parser with a diagnostic 
  message instead of silently dropping data
- SourceAttributes, FeatureAttributes, and SequenceAttributes converted from enums 
  to structs; fields are now accessed directly instead of via pattern matching
- All optional fields on attribute structs are now typed as Option<T> reflecting their 
  true presence/absence semantics in GenBank/EMBL files
- We had two types of Records (gbk and embl) after parsing, they have been converted to one type of records after parsing - several structures and functions have been moved from gbk.rs and embl.rs into record.rs allowing a generic records type to be used for both
- Heatmap path fix
- Moved images folder to assets in docs windows install section

## 2026
Since 0.1.4 (Aug – Oct 2026)
Security
Applied security fixes to all GitHub workflows. (#56) by @ubiratansoares
Performance
Improved the efficiency of the seqmetrics instability index calculation (closes #52). (#55) by @ChinmayMoghe
Documentation
Added a DeepWiki link for AI-generated docs. (#54) by @colindaven
0.1.4 (Jun 2, 2026)
Release
Version 0.1.4 final. (#49) by @LCrossman
Documentation
Added an example of #[pyclass] usage (#38), building on the original proposal by @davidhewitt. (#45) by @LCrossman
Python bindings (microbiorust-py)
Added pytests and improved __init__.py setup. (#41) by @LCrossman
Fixed top-level imports and added __init__.py to support them. (#39) by @LCrossman
Packaging
Merged changes for pip and crates releases. (#37) by @LCrossman
2025
December 2025
Changed
Compared and merged changes across forks. (#36) by @LCrossman
Performance
GenBank (GBK) parsing speedup. (#35) by @LCrossman
November 2025
Performance
Optimised parser performance by pre-compiling regexes with lazy_static. (#34) by @kh3rld
Documentation
Added missing trailing code fences in the crates README. (#32) by @corneliusroemer
Maintenance
Incorporated changes following the GitHub organisation move. (#31) by @LCrossman
September 2025
Changed
Refactored embl, gbk and record: deduplicated RangeValue, introduced GenericRecord and conversions, and exposed the record module. (#30) by @kh3rld
Documentation
README update. (#29) by @ZEZE1020
August 2025
Changed
Reusable code improvements. (#26) by @martcpp
July 2025
Changed
Updated use declarations. (#25) by @pem131
Cleaned up the repository. (#23) by @martcpp
June 2025
Documentation
Fixed the repository link and made small formatting fixes. (#21) by @CGMossa
March 2025
Changed
Refactored the heatmap module and improved its errors. (#16) by @em-baggie
2024
December 2024
Changed
Improved error handling in the heatmap. (#14) by @EleisonC
Improved error handling in the read function. (#13) by @EleisonC
October 2024
Fixed
Reviewed and corrected CamelCase / snake_case naming. (#10) by @Pranith2003
Contributors

Thank you to everyone who has contributed!:

@LCrossman (owner), @ubiratansoares, @ChinmayMoghe, @colindaven, @davidhewitt, @kh3rld, @corneliusroemer, @ZEZE1020, @martcpp, @pem131, @CGMossa, @em-baggie, @EleisonC, @Pranith2003

