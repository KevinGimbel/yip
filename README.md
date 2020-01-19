# yip
> Tiniest server which responds with requester IPv4 address
<!-- BEGIN mktoc -->
- [Why](#why)
- [Usage](#usage)
  - [Local](#local)
  - [Docker](#docker)
- [Contributing](#contributing)
  - [Where to start?](#where-to-start?)
- [Code of Conduct](#code-of-conduct)
- [License](#license)
<!-- END mktoc -->

## Why
[⬆️ Back to Top](#table-of-contents)

This server is the counter part to [https://github.com/kevingimbel/mip](https://github.com/kevingimbel/mip).

It responds with the callers' IP address on request. That's it.

## Usage
[⬆️ Back to Top](#table-of-contents)

### Cargo

Run the server with `cargo run`, then connect to it on port 8111.

### Docker

Start the Docker container and connect on port `8111` (or what ever port chosen).

```sh
docker run --rm -p 8111:8111 kevingimbel/mip
```

Using `docker-compose`:

```yaml
version: '3'

services:
    mip:
        image: kevingimbel/mip
        ports:
            - 8111:8111
``` 

Then start with `docker-compose up -d`.

## Contributing
[⬆️ Back to Top](#table-of-contents)

We love and welcome every form of contribution!

### Where to start?

Here are some good places to start:

* Issues with label [Good first issue](https://github.com/kevingimbel/yip/labels/good%20first%20issue)
* Issues with label [Documentation](https://github.com/kevingimbel/yip/labels/documentation)
* Providing example implementations or usage demos

See [open issues](https://github.com/kevingimbel/yip/issues).


## Code of Conduct
[⬆️ Back to Top](#table-of-contents)

You are expected to follow our [code of conduct](https://github.com/kevingimbel/yip/blob/master/CODE_OF_CONDUCT.md) when interacting with the project via issues, pull requests or in any other form. Many thanks to the awesome [contributor covenant](https://www.contributor-covenant.org/) initiative!

## License
[⬆️ Back to Top](#table-of-contents)

GNU AFFERO GENERAL PUBLIC LICENSE 3, see LICENSE file.