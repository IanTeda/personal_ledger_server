This git repository is for REST Server component of [Personal Ledger](https://github.com/IanTeda/personal_ledger) 

## Summary


## Features


## Usage


## Getting Started


### Pull Submodule

Pull the latest server commits

```bash
git submodule foreach git pull origin main
```

## Code Structure

The code for this API is structured around:

1. Application code
2. Routing code
3. Handlers code
4. Services code
5. Models (Maybe)

### 1. Application Code

Application code sits at root of the src file and includes code for the following:

* Main function
* Configuration
* Startup
* Telemetry
* Utilities or Helpers

### 2. Routing Code

Routing code abstracts the Actix routing configuration code. They are configured
to use handler functions.

### 3. Handlers Code

Handlers process http requests and provide a response. They use services to create
and fetch data from database. 

### 4. Services Code

Services create and make the data requests from the database.

#### References

* [Making Great Docs with Rustdoc](https://www.tangramvision.com/blog/making-great-docs-with-rustdoc)
* [The Rust Programming Language - Documentation](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/documentation.html)
* [Guide on how to write documentation for a Rust crate](https://blog.guillaume-gomez.fr/articles/2020-03-12+Guide+on+how+to+write+documentation+for+a+Rust+crate)