### Folder structure:
Every Folder contains a mod.rs, which in turn contains the named submodule. That way the files are searchable and the tabs are not all called "mod.rs".

### What is different to ourstructure?
The commandhandler trait has been made async with the nightly async trait. This can even be used outside of the business project. The example for this is in the controllerservice. Although it cannot be used in repository - clinic_repo, without declaring the nightly feature in the lib file...

### Questions
Why can i implement the async trait in controllerservice, but not in clinic_repo, without declaring the asynctrait nightly version in lib?