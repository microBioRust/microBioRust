**How to Install**

**Install Rust (recommended method):**

Please see [here](https://www.rust-lang.org/tools/install) for base environment installation details for Rust programming language 

For MacOSX, Linux or other unix OS you can install Rust _via_:

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

For further details on Windows please see: [other install](https://forge.rust-lang.org/infra/other-installation-methods.html)

**Install with Conda:**  
*Disclaimer:* While conda is a powerful package manager, it is primarily designed for managing Python environments, and the 
Rust package available may not always be up-to-date or include all the necessary components

```conda install conda-forge::rust```


**Install with Python for Rust with Python InterOp**  
To use Python Interoperability you will need to install Python and additionally you may need (even if in a conda environment):

For Linux:
```export LD_LIBRARY_PATH=[directory where python is located]``` 

For MacOSX: 
```export DYLD_LIBRARY_PATH=[directory where python is located]```

For Windows please see this StackOverflow issue for a fix: [fix here](https://stackoverflow.com/questions/79627918/cant-set-python-version-when-running-rust-analyzer-and-pyo3-on-wsl/79627921#79627921) and please also see our specific windows_install page






