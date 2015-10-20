# BASMAP
Buntine's Awesome Sitemap Audit Program.

BASMAP is a command line utility for auditing the [Sitemap](https://en.wikipedia.org/wiki/Sitemaps) at a given website.

## Usage

```
basmap URL [options]

Options:
    -c NUMBER           Amount of concurrent HTTP requests (default 5)
    -s NUMBER           Milliseconds to sleep between requests (default 1000)
    -h, --help          Print this help menu
    -r, --redirects     Consider HTTP redirects (30x) successful
    -v, --verbose       Print progress verbosely
```

# Example

```
$ basmap http://news.com.au/sitemap.xml -c 20 -s 2000 -r
```
