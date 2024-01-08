# jason.json

This is a dynamic resume site. Content is pulled from [a JSON file](https://github.com/Elgenzay/jason.json/blob/main/static/data/sample.json) and rendered with [Tera](https://crates.io/crates/tera).


## Setup
Clone and build:  
`cd` to your preferred project directory and run:
```sh
$ git clone git@github.com:Elgenzay/jason.json.git
$ cd jason.json
$ cargo build --release
```

Alternatively, you can build an image from the provided Dockerfile.

- Create `/static/favicon.ico`
- Create `/static/data/{YOUR_NAME}.json`, using the [sample](https://github.com/Elgenzay/jason.json/blob/main/web/static/data/sample.json) as a reference.


- Create `.env`:
```env
FILE_NAME = "{YOUR_NAME}.json"
DOMAIN = "{EXAMPLE.COM}"
```
`FILE_NAME` is required. `DOMAIN` is required if using TLS.  
If you're going to add data that you want obfuscated to your .json file, you can add that here.

Example:
```env
EMAIL = "{OPTIONAL_EMAIL}"
```
To obfuscate a contact field, use the environment variable name in the "env_var" field of the contact item,  
as demonstrated in the [sample JSON file](https://github.com/Elgenzay/jason.json/blob/main/web/static/data/sample.json):
```JSON
{
	"label": "Email",
	"env_var": "EMAIL"
}
```
Note that `FILE_NAME` can be overridden with the `file` url query parameter.  
`file` must be alphanumeric, and match the name of a json file in `web/static/data/`.  
For instance, `http://127.0.0.1/?file=sample` will use data from `web/static/data/sample.json`.
