# clipboard

Copy files and text through network.
Main lowlevel logic from [FreeRDP](https://github.com/FreeRDP/FreeRDP).

TODO: Move this lib to a separate project.

## impl

We propose a two layered clipboard implementation, a wired layer for data transmission
and adapter implementations for different platforms.

### Wired Layer

This layer includes a protocol implementation similar to
Clipboard Virtual Channel Extension of RDP, but running on rustdesk's protobuf channel.
The protocol is built on top of the adapter typeclass.

### Native Layer

This layer include specific implementation for different platforms,
like X11, Windows and MacOS.

### windows

![scene](./docs/assets/scene3.png)

![A1->B1](./docs/assets/win_A_B.png)

![B1->A1](./docs/assets/win_B_A.png)
