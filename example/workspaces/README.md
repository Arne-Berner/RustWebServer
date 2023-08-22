### How Cargo can be used:
To run a specific project in your workspace from your root folder use:  
cargo run -p PROJECTNAME  
To run the binary in a workspace use cargo run.  
If there is more than one binary in the workspace, cargo run will throw an error. 

### What is the workspace?
A workspace can be used similarly to mods, so that only one cargo.lock and one target will be produced. But instead of using mods, it uses libraries.  
It will specify its members by using a cargo.toml which has no package or dependencies section, but has a [workspace] section instead. The benefit of using libraries instead of modules is that you can swap the libraries and share the libraries between applications. You can also host your libraries on crate.io. But beware, libraries on crate.io cannot be deleted.

### What are dependencies?
You can specify your dependencies in your binary cargo.toml. Dependencies need to be libraries and can be reused by a lot of other binaries. A binary cannot be a dependency.

### What are modules and submodules?
Modules are like namespaces in other languages. They are declared within main and can only be found in the same folder as main, as an inline module in main.rs itself or in subfolders in the same folders containing a mod.rs file. Submodules are declared in the same matter in modules. Test shows this. 

### What could be an example structure?
We could have one root composite binary, which will use a webframework to create an entry function: main. This binary will depend on a controller, a business layer (with our business specific functions) and a datalayer (with a connection to our postgres server via seaORM). Those three dependencies will be in their own libraries, so that they can be changed and reused at will.
