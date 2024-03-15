# Personal Ledger - Server 

This git repository is for REST Server component of [Personal Ledger](https://github.com/IanTeda/personal_ledger) 

## Pull Submodule

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