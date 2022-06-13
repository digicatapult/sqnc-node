# Doas Module

## Overview

The Doas module allows for a configurable `Origin`
to execute dispatchable functions that require a `Root` call.
This is seen as a more flexible of the [`sudo`](https://docs.rs/pallet-sudo/latest/pallet_sudo)
pallet provided by ParityTech. This pallet may be used in conjunction with the
[`collective`](https://docs.rs/pallet-sudo/latest/pallet_collective) pallet
to enable `sudo` like functionality where a majority of the collective
must agree to perform the action.

## Interface

### Dispatchable Functions

Only the sudo key can call the dispatchable functions from the Sudo module.

* `doas_root` - Make a `Root` call to a dispatchable function
* `doas_root_unchecked_weight` - Make a `Root` call to a dispatchable function overriding weight calculations
* `doas` - Make a call as a specific user `Origin` to a dispatchable function

## Usage

### Executing Privileged Functions

The Doas module itself is not intended to be used within other modules.
Instead, you can build "privileged functions" (i.e. functions that require `Root` origin) in other modules.
You can execute these privileged functions by dispatching `doas` form the configured `Origin`.
Privileged functions cannot be directly executed via an extrinsic.

Learn more about privileged functions and `Root` origin in the [`Origin`] type documentation.

### Simple Code Snippet

This is an example of a module that exposes a privileged function:

```rust
use frame_support::{decl_module, dispatch};
use frame_system::ensure_root;

pub trait Config: frame_system::Config {}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
        pub fn privileged_function(origin) -> dispatch::DispatchResult {
            ensure_root(origin)?;

            // do something...

            Ok(())
        }
    }
}
```

## Related Modules

* [Sudo](https://docs.rs/pallet-sudo/latest/pallet_sudo)
* [Collective]((https://docs.rs/pallet-sudo/latest/pallet_collective)
* [Democracy](https://docs.rs/pallet-democracy/latest/pallet_democracy/)

# Attribution

`pallet-doas` has been adapted from the version `3.0.0` of the
[`Sudo`](https://docs.rs/pallet-sudo/latest/pallet_sudo) pallet originally
licensed under the Apache-2.0 license. A copy of this license can
be found at [./licenses/pallet-sudo.LICENSE](./licenses/pallet-sudo.LICENSE).
This module is then relicensed by Digital Catapult under the same Apache-2.0
license a copy of which is [in the root of this repository](../../LICENSE)

License: Apache-2.0
