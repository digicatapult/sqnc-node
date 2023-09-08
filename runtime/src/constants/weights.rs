use frame_support::weights::{constants, RuntimeDbWeight};
use sp_core::parameter_types;

parameter_types! {
    /// By default, Substrate uses `RocksDB`, so this will be the weight used throughout
    /// the runtime.
    pub const RocksDbWeight: RuntimeDbWeight = RuntimeDbWeight {
    /// Time to read one storage item.
    /// Calculated by multiplying the *Average* of all values with `1.0` and adding `0`.
    ///
    /// Stats nanoseconds:
    ///   Min, Max: 4_030, 54_531
    ///   Average:  7_338
    ///   Median:   5_630
    ///   Std-Dev:  7759.74
    ///
    /// Percentiles nanoseconds:
    ///   99th: 54_531
    ///   95th: 9_000
    ///   75th: 6_851
    read: 7_338 * constants::WEIGHT_REF_TIME_PER_NANOS,

    /// Time to write one storage item.
    /// Calculated by multiplying the *Average* of all values with `1.0` and adding `0`.
    ///
    /// Stats nanoseconds:
    ///   Min, Max: 15_450, 2_179_245
    ///   Average:  87_849
    ///   Median:   28_621
    ///   Std-Dev:  339729.37
    ///
    /// Percentiles nanoseconds:
    ///   99th: 2_179_245
    ///   95th: 126_782
    ///   75th: 35_410
    write: 87_849 * constants::WEIGHT_REF_TIME_PER_NANOS,
    };
}

#[cfg(test)]
mod test_db_weights {
    use super::constants::RocksDbWeight as W;
    use frame_support::weights::constants;

    /// Checks that all weights exist and have sane values.
    // NOTE: If this test fails but you are sure that the generated values are fine,
    // you can delete it.
    #[test]
    fn bound() {
        // At least 1 µs.
        assert!(
            W::get().reads(1).ref_time() >= constants::WEIGHT_REF_TIME_PER_MICROS,
            "Read weight should be at least 1 µs."
        );
        assert!(
            W::get().writes(1).ref_time() >= constants::WEIGHT_REF_TIME_PER_MICROS,
            "Write weight should be at least 1 µs."
        );
        // At most 1 ms.
        assert!(
            W::get().reads(1).ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
            "Read weight should be at most 1 ms."
        );
        assert!(
            W::get().writes(1).ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
            "Write weight should be at most 1 ms."
        );
    }
}
