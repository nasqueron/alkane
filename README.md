# Alkane

## Features

The `alkane` command allows to manage the Nasqueron PaaS Alkane,
to host PHP sites through nginx and php-fpm, and static sites
directly through nginx.

It can work in two complementary modes:

  - as a command, to allow local system administration task and debug
  - as a server, to allow components to interact with it for automation

For example, to update the website foo.domain.tld, you can:

  - use the command `alkane update foo.domain.tld`
  - send a POST request to `http://localhost:10206/update/foo.domain.tld`

For example, you can use the server to allow Jenkins or your continuous
delivery system to trigger an update process.

Both modes will run the same action code.

## Usage

### Alkane update

The update feature allows to update a site according configured instructions:

  - do something simple like `git pull`
  - download a Jenkins artifact
  - run a recipe like `git pull && composer update` or `make update`
  - it can ask php-fpm to reload, so Opcache picks the new code

The instructions aren't given as command argument or HTTP request, but
preconfigured on the server.

Alkane is responsible to update the local site content, to help deploying
your artefact, but not to update nginx or certificates configuration.
On Nasqueron servers, nginx is provisioned through Salt.

When triggered from HTTP API by an external service like CD,
the raw POST content is available to the instructions script
under ALKANE_SITE_CONTEXT environment variable.

### Alkane init

The init feature works similarly than the update process, but is responsible
to deploy the site the first time, when it doesn't exist.

### Alkane misc commands

  - **is-present**: determine if a site is hosted on the PaaS
  - **deploy**: call `init` or `update` as needed

### Alkane server

To run the **Alkane** server and expose the API, use `alkane server`.

The following environment variables allow to configure the server:

| Variable           | Description     | Default value |
|--------------------|-----------------|---------------|
| ROCKET_PORT        | Server port     | 8000          |
| ROCKET_ADDRESS     | Address to bind | 0.0.0.0       |

The following options allow to configure the server:

| Argument           | Description     | Default value |
|--------------------|-----------------|---------------|
| --mounting-point   | Mounting point  | /             |

Nasqueron servers expose Alkane on the port 10206, for the alkane C2H6.

## Configuration

### Configuration file

The configuration is written in YAML and is seeked at:

- .alkane.conf
- /usr/local/etc/alkane.conf
- /etc/alkane.conf

This repository provides an example `.alkane.conf` both for reference
and for testing suite: path matches the test/data folder.

### Recipes scripts

Each site should have two scripts in /usr/local/libexec/alkane/<site name>,
or any other directory set as `roots.recipes` in the configuration:

  - init: called by `alkane init`
  - update: called by `alkane update`

Several environment variables are available to those scripts:

| Variable            | Description                                |
|---------------------|--------------------------------------------|
| ALKANE_RECIPES_PATH | The *root* path to the recipes             |
| ALKANE_SITE_NAME    | The site name, for example the FQDN        |
| ALKANE_SITE_PATH    | The full path to the site content          |
| ALKANE_SITE_CONTEXT | Arbitrary context sent to HTTP API, if set |

The following security consideration should be exercised:

  - if you symlink or set the recipes root to a file in the site repository,
    you trust the site to run arbitrary code as the alkane user
  - don't trust blindly context information, it can be false or malformed,
    and is a vector for attack if your CD is compromised
  - if you run the server HTTP API, listen only to private IP address,
    or use a firewall to block the port, it shouldn't be reachable publicly

## Development notes
### Design goals

Alkane update process helps us to separate concerns:

  - Infrastructure as code configure the server and provisions Alkane config
  - Both Salt and Jenkins can when needed request a site update
  - CD prepare the build (npm build for example) and ask Alkane to deploy it

At Nasqueron, we wanted to get a more standardized way to work and ensure
the site ends in the same state if deployed through Salt or Jenkins CD.

Currently, Alkane isn't responsible to rollback the deployment. To rollback,
you can revert the commit, and trigger CD for it and Alkane will pick it
like usual.

Alkane can help you to run Canary tests. To do so, you can run `alkane update`
on a server, or a small range of servers, observe traffic,  and take the
decision to deploy everywhere from responses from those.

### License

(c) 2023, Sébastien Santoro, Nasqueron, some rights reserved.
Released under BSD-2-Clause license. See [license](LICENSE).

### Contribute

Alkane is written in Rust using:

  - Rocket and Limiting Factor for the HTTP API
  - Clap to parse arguments
  - serde_yaml to deserialize configuration files

Run `make` to build the first time: it will download public suffix list
and then run `cargo build`. Afterwards, you can directly use cargo.

Issues can be tracked at https://devcentral.nasqueron.org/

To send your change, you can follow this guide:
https://agora.nasqueron.org/How_to_contribute_code
