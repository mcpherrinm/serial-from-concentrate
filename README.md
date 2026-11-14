# Serial From Concentrate

This is the top-level folder tracking serial-from-concentrate work.

The name's a work in progress.

There are a few dirs here:

## Firmware

The RP235x firmware which talks serial.
Based on Embassy-rs.

## Controller

The controller runs on Linux. It connects to multiple serial devices via one of
the supported mechanisms. It handles logging output, managing multiple incoming
connected users via SSH or API, and serves a Web UI.

Typically it is deployed either in the 1U appliance or a standalone server.

The Web UI developement might be here, TBD.

## Others to come

We'll have the hardware designs, starting with the standalone USB dongle.

