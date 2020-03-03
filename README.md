# seed_hooks: Local state for seed apps

seed_hooks is a crate that allows you to store state on a per component basis in your seed apps.
It is designed as a clone of React Hooks, principally the useState hook.

Here a component is defined as a 'topological aware execution context', this 
means that the component is aware of its own call-site identity and location
in the call tree.

**Example:**

This is a complete counting button with state implemented in in the Seed framework:

```rust
use seed_hooks::*;

#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 0);

    div![
        count,
        button![count.mouse_ev(Ev::Click, |count, _| *count += 1), "Click me"],
    ]
}
```

The two most important functions are:
 
* use_state(|| .. ) stores component state for the type returned by the closure. 
  Returns a state accessor. 
* `#[topo::nested]` function annotation defines the a topologically aware function. Everything 
  executed within the function will have its own unique topological id. The outermost nested function
  acts as a "root" which resets the topology and enables specific components to have
  a "topological identity".


**How does it work?**

- this relies on the `#![feature(track_caller)]` feature gate to be enabled.

- topo creates a new execution context for every `#[topo::nested]` function or every `topo::call` block. The outermost call
re-roots the execution context. The re-rooting allows for consistent 
execution contexts for the same components as long as you re-root at the start of the 
base view function. This means that one can store and retrieve local data for an 
individual component annotated by `#[topo::nested]`.

- The execution context is not only determined by the order of calling a  
functions but also the source location of these calls. This means that state is 
consistent and stable even though branching logic might call topologically 
aware functions in different orders.

- See this awesome talk explaining how topo works: https://www.youtube.com/watch?v=tmM756XZt20

- a type gets stored with : `let my_string  = use_state(||"text")` 
which stores `"text"` in the component for the `str` type. This returns a 
 state accessor struct responsible for getting and setting of the state.

- The accessor is useful because it can be passed to callbacks or cloned or called from 
different topological contexts. i.e. `my_string.(new_text)` will work no matter 
where it is called.

- currently seed_hooks exposes a clone to stored values via `get()` and to non-Clone types with `get_with()`

- seed_hooks is a Seed specific wrapper around the comp_state crate.

