Morsadio
========

Small CLI program that speaks morse code.

## Usage

For example, following command will speak "SOS" in 999 hertz twice.

```bash
morsadio -z 999 "... --- ... / ... --- ..."
```

## Options

`-z | --hertz` - How should program beep in hertz.

`-s | --short-beep-ms` - How long should `.` be speaked in milliseconds.

`-l | --long-beep-ms` - How long should `-` be speaked in milliseconds.
