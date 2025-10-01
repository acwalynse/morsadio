Morsadio
========

Small CLI program that reproduces morse code.

## Usage

For example, following command will reproduce "SOS" in 999 hertz twice.

```bash
morsadio -z 999 "... --- ... / ... --- ..."
```

## Options

`-z | --hertz` - How beep should sound in hertz.

`-s | --short-beep-ms` - How long should "." sound in milliseconds.

`-l | --long-beep-ms` - How long should "-" sound in milliseconds.
