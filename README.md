# sblex-rs

## Memory usage

Morphology is the beast in the room that uses a lot of memory.

Python reference:
![Diagram showing memory usage of Python version](assets/images/python_baseline_memory_usage.png)

Baseline:

![Diagram showing memory usage of load_morphology](assets/images/rust_baseline_memory_usage.png)

### Use ArcStr instead of String

We can replace `String` with `arcstr::ArcStr` as value in the `Trie` to reduce memory usage.

![Diagram showing memory usage of load_morphology](assets/images/rust_use_arcstr_as_value_memory_usage.png)

And if also use `ArcStr` as keys in the inner map we can reduce it some more:

![Diagram showing memory usage of load_morphology](assets/images/rust_use_arcstr_as_key_memory_usage.png)

### Decomposing builder

![Diagram showing memory usage of load_morphology](assets/images/rust_decompose_builder_memory_usage.png)