use frame_support::Parameter;
use sp_std::vec::Vec;
use vitalam_pallet_traits::ProcessIO;

#[cfg_attr(feature = "std", derive(Debug))]
pub enum Restriction {
    None,
    SenderOwnsAllInputs,
    FixedNumberOfInputs { num_inputs: u32 },
}

pub fn validate_restriction<A, R, T, V>(
    restriction: &Restriction,
    sender: &A,
    inputs: &Vec<ProcessIO<A, R, T, V>>,
    _outputs: &Vec<ProcessIO<A, R, T, V>>,
) -> bool
where
    A: Parameter + Default,
    R: Parameter + Default + Ord,
    T: Parameter + Default + Ord,
    V: Parameter + Default,
{
    match *restriction {
        Restriction::None => true, // TODO implement some actual restrictions
        Restriction::FixedNumberOfInputs { num_inputs } => return inputs.len() == num_inputs as usize,
        Restriction::SenderOwnsAllInputs => {
            for input in inputs {
                let is_owned = match input.roles.get(&Default::default()) {
                    Some(account) => sender == account,
                    None => false,
                };
                if !is_owned {
                    return false;
                }
            }
            true
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use sp_std::collections::btree_map::BTreeMap;

    #[test]
    fn no_restriction_succeeds() {
        let result = validate_restriction::<u64, u32, u32, u64>(&Restriction::None, &1u64, &Vec::new(), &Vec::new());
        assert!(result);
    }

    #[test]
    fn sender_owns_inputs_restriction_no_inputs_succeeds() {
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::SenderOwnsAllInputs,
            &1u64,
            &Vec::new(),
            &Vec::new(),
        );
        assert!(result);
    }

    #[test]
    fn sender_owns_inputs_restriction_owns_all_inputs_succeeds() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(result);
    }

    #[test]
    fn sender_owns_inputs_restriction_owns_no_inputs_fails() {
        let mut is_not_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_not_owner.insert(Default::default(), 2u64);
        let inputs = vec![
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(!result);
    }

    #[test]
    fn sender_owns_inputs_restriction_owns_some_inputs_fails() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut is_not_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_not_owner.insert(Default::default(), 2u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(!result);
    }

    #[test]
    fn sender_owns_inputs_restriction_incorrect_role_fails() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut is_not_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_not_owner.insert(1u32, 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(!result);
    }

    #[test]
    fn matches_fixed_input_total() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfInputs { num_inputs: 2 },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(result);
    }

    #[test]
    fn matches_fixed_input_total_two() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfInputs { num_inputs: 4 },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(result);
    }

    #[test]
    fn matches_fixed_input_total_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfInputs { num_inputs: 1 },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(!result);
    }

    #[test]
    fn matches_fixed_input_total_fail_two() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfInputs { num_inputs: 3 },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(!result);
    }
}
