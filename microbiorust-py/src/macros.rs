#[macro_export]
macro_rules! impl_build_features {
    ($fn_name:ident, $record_type:ty, $attrs_path:path) => {
        fn $fn_name(
            r: &$record_type,
            py: ::pyo3::Python<'_>,
        ) -> ::std::collections::HashMap<String, ::pyo3::Py<$crate::PyFeatureInfo>> {
            let mut map = ::std::collections::HashMap::new();
            for (tag, attrs) in &r.cds.attributes {
                let tag_owned = tag.clone();
                let mut info = $crate::PyFeatureInfo::new(&tag_owned);
                info.gene = attrs.gene.clone();
                info.product = attrs.product.clone();
                info.start = attrs.start.as_ref().map(|v| v.get_value());
                info.stop = attrs.stop.as_ref().map(|v| v.get_value());
                info.strand = attrs.strand;
                info.codon_start = attrs.codon_start;
                if let Some(t) = &attrs.transl_table {
                    info.extras.push(format!("transl_table: {:?}", t));
                }
                if let Some(k) = &attrs.feature_type {
                    info.extras.push(format!("feature_type: {:?}", k));
                }
                let obj = ::pyo3::Py::new(py, info).expect("failed to allocate the PyFeatureInfo");
                map.insert(tag_owned, obj);
            }
            map
        }
    };
}
#[macro_export]
macro_rules! impl_build_sequences {
    ($fn_name:ident, $record_type:ty, $attrs_path:path) => {
        fn $fn_name(
            r: &$record_type,
            py: ::pyo3::Python<'_>,
        ) -> ::std::collections::HashMap<String, ::pyo3::Py<$crate::PySequenceInfo>> {
            let mut map = ::std::collections::HashMap::new();
            for (tag, attrs) in &r.seq_features.seq_attributes {
                let tag_owned = tag.clone();
                let mut info = $crate::PySequenceInfo::new(&tag_owned);
                info.faa = attrs.sequence_faa.clone();
                info.ffn = attrs.sequence_ffn.clone();
                if info.faa.is_some() || info.ffn.is_some() {
                    let obj =
                        ::pyo3::Py::new(py, info).expect("failed to allocate the PySequenceInfo");
                    map.insert(tag_owned, obj);
                }
            }
            map
        }
    };
}
#[macro_export]
macro_rules! impl_build_faa {
    ($fn_name:ident, $record_type:ty, $enum_path:path) => {
        fn $fn_name(
            r: &$record_type,
            py: ::pyo3::Python<'_>,
        ) -> ::std::collections::HashMap<String, ::pyo3::Py<$crate::PyFaaInfo>> {
            let mut map = ::std::collections::HashMap::new();
            for (tag, attrs) in &r.seq_features.seq_attributes {
                if let Some(faa_seq) = attrs.sequence_faa.clone() {
                    let info = $crate::PyFaaInfo {
                        locus_tag: tag.clone(),
                        faa: faa_seq,
                    };
                    let obj = ::pyo3::Py::new(py, info).expect("failed to allocate a PyFaaInfo");
                    map.insert(tag.clone(), obj);
                }
            }
            map
        }
    };
}

#[macro_export]
macro_rules! impl_build_ffn {
    ($fn_name:ident, $record_type:ty, $enum_path:path) => {
        fn $fn_name(
            r: &$record_type,
            py: ::pyo3::Python<'_>,
        ) -> ::std::collections::HashMap<String, ::pyo3::Py<$crate::PyFfnInfo>> {
            let mut map = ::std::collections::HashMap::new();
            for (tag, attrs) in &r.seq_features.seq_attributes {
                if let Some(ffn_seq) = attrs.sequence_ffn.clone() {
                    let info = $crate::PyFfnInfo {
                        locus_tag: tag.clone(),
                        ffn: ffn_seq,
                    };
                    let obj = ::pyo3::Py::new(py, info).expect("failed to allocate a PyFfnInfo");
                    map.insert(tag.clone(), obj);
                }
            }
            map
        }
    };
}
#[macro_export]
macro_rules! create_collection {
    ($struct_name:ident, $item_type:ty, $label:expr) => {
        #[::pyo3_stub_gen::derive::gen_stub_pyclass]
        #[pyclass]
        pub struct $struct_name {
            // Use absolute paths so it works in any module
            pub inner: ::std::collections::HashMap<String, ::pyo3::Py<$item_type>>,
        }
        #[::pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pymethods]
        impl $struct_name {
            fn __len__(&self) -> usize {
                self.inner.len()
            }

            fn __contains__(&self, tag: &str) -> bool {
                self.inner.contains_key(tag)
            }

            fn __repr__(&self) -> String {
                format!("{}({} entries)", $label, self.inner.len())
            }

            fn __getitem__<'py>(
                &self,
                tag: &str,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, $item_type>> {
                self.inner
                    .get(tag)
                    .map(|obj| obj.bind(py).clone())
                    .ok_or_else(|| {
                        let available = self
                            .inner
                            .keys()
                            .take(5)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ");
                        ::pyo3::exceptions::PyKeyError::new_err(format!(
                            "locus tag {} is not found. The first 5 available tags are: {},\n
                            Use list(collection.keys()) to see all tags.",
                            tag, available
                        ))
                    })
            }
            fn values<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyList>> {
                let values: Vec<_> = self
                    .inner
                    .values()
                    .map(|obj| obj.bind(py).clone())
                    .collect();
                ::pyo3::types::PyList::new(py, values)
            }
            fn items<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyList>> {
                let items: Vec<::pyo3::Bound<'py, ::pyo3::types::PyTuple>> = self
                    .inner
                    .iter()
                    .map(|(k, v)| {
                        ::pyo3::types::PyTuple::new(
                            py,
                            &[
                                k.clone().into_pyobject(py).unwrap().into_any(),
                                v.bind(py).clone().into_any(),
                            ],
                        )
                    })
                    .collect::<::pyo3::PyResult<Vec<_>>>()?;
                ::pyo3::types::PyList::new(py, items)
            }
            fn keys<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyList>> {
                ::pyo3::types::PyList::new(py, self.inner.keys())
            }

            fn __iter__(
                slf: ::pyo3::PyRef<'_, Self>,
            ) -> ::pyo3::PyResult<::pyo3::Py<$crate::LocusTagIterator>> {
                let iter = $crate::LocusTagIterator {
                    keys: slf.inner.keys().cloned().collect(),
                    index: 0,
                };
                ::pyo3::Py::new(slf.py(), iter).map_err(|e| e.into())
            }
            /// Serializes the collection to a formatted JSON string
            fn to_json<'py>(&self, py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<String> {
                // 1. Create a pure Rust HashMap
                let mut export_map = ::std::collections::HashMap::new();

                // 2. Extract the pure Rust data from the Py wrappers
                for (key, py_ptr) in &self.inner {
                    let rust_struct = py_ptr.bind(py).borrow().clone();
                    export_map.insert(key.clone(), rust_struct);
                }
                // 3. Serialize using serde_json
                ::serde_json::to_string_pretty(&export_map).map_err(|e| {
                    ::pyo3::exceptions::PyValueError::new_err(format!(
                        "JSON serialization failed: {}",
                        e
                    ))
                })
            }
        }
    };
}
//register the functions on submodules and parent
#[macro_export]
macro_rules! register_all {
    ($sub:expr, $parent:expr, $($func:ident),+ $(,)?) => {
        $(
            $sub.add_function(::pyo3::wrap_pyfunction!($func, &$sub)?)?;
            $parent.add_function(::pyo3::wrap_pyfunction!($func, $parent)?)?;
        )+
    };
}
//only register the functions on the submodules
//#[macro_export]
//macro_rules! register_functions {
//    ($module:expr, $($func:ident),*) => {
//        $(
// Using the full path pyo3::wrap_pyfunction ensures it always resolves
//            $module.add_function(pyo3::wrap_pyfunction!($func, &$module)?)?;
//        )*
//    };
//}
