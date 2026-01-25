## Sum Task Example

This example demonstrates how to use the Sum task in Mate to calculate the sum of
multiple numbers.

### Usage

1. Build the task using the Justfile recipe as follows:

```bash
just build-task sum
```

2. Run the task with the required arguments:

```bash
mate task run \
--source ./sum.wasm \
--args '[1, 2, 3, 4, 5]'
```
