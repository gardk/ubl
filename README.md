# (U)nbound (B)lock(l)ist

This program takes hostsfile formatted input and spits out `local-zone` entries for each of them.

```sh
parallel <blocklist-urls.txt curl -sf | cargo run -rq > blocklist.conf
```

