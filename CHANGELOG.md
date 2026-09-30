# Changelog #
---

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project should adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## Unreleased ##
---

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

### Removed ###


