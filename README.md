# whereami

A simple cloudflare worker that uses cloudflare to determine the city or coordinates of a user

## Usage

```bash
curl https://whereami.jtmiclat.me/
# Tokyo, Japan
```

```bash
curl https://whereami.jtmiclat.me/coordinates
# [139.6899, 35.6893]
```

## Why this exists

I wanted a simple way of testing what country I set my vpn to using a terminal
