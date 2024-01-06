# jason.json

This is a dynamic resume site. Content is pulled from [a JSON file](https://github.com/Elgenzay/jason.json/blob/main/web/static/data/sample.json) and rendered with [Tera](https://crates.io/crates/tera).


## Unix setup
Clone and build:  
`cd` to your preferred project directory and run:
```sh
$ git clone git@github.com:Elgenzay/jason.json.git
$ cd jason.json
$ cargo build --release
```

- Create `favicon.ico`
- Create `web/static/data/{YOUR_NAME}.json`, using the [sample](https://github.com/Elgenzay/jason.json/blob/main/web/static/data/sample.json) as a reference.


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

## Configure TLS (optional)

### With TLS:

[Get your certificate](https://certbot.eff.org/) and run (from project root):
```sh
$ cd tls/
$ ln -s /etc/letsencrypt/live/{DOMAIN}/fullchain.pem fullchain.pem
$ ln -s /etc/letsencrypt/live/{DOMAIN}/privkey.pem privkey.pem
```

### Without TLS:
Change the contents of `web/Rocket.toml` to:

```toml
[default]
port = 80

[rocket_dyn_templates]
dir = "content"
```


## Create services
This section assumes a project directory at `/jason.json`.

Create `/etc/systemd/system/web.service`:
```
[Unit]
Description=Web
StartLimitBurst=5
StartLimitIntervalSec=0

[Service]
User=root
WorkingDirectory=/jason.json/web
Environment="ROCKET_PROFILE=production"
Environment="ROCKET_ADDRESS={IP_HERE}"
Environment="ROCKET_LOG=critical"
ExecStart=/jason.json/target/release/web
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```


Create `/etc/systemd/system/https_redirect.service` (unless running without TLS):
```
[Unit]
Description=HTTPS Redirect
StartLimitBurst=5
StartLimitIntervalSec=0

[Service]
User=root
WorkingDirectory=/jason.json/https-redirect
Environment="ROCKET_PROFILE=production"
Environment="ROCKET_ADDRESS={IP_HERE}"
Environment="ROCKET_LOG=critical"
ExecStart=/jason.json/target/release/https-redirect
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Start the web service:
```sh
$ systemctl enable web
$ systemctl start web
```

If using TLS, also run:
```sh
$ systemctl enable https_redirect
$ systemctl start https_redirect
```
