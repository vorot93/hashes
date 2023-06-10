use digest::{dev::fixed_test, new_test};

new_test!(blake224, "blake224", blake::Blake224, fixed_test);
new_test!(blake256, "blake256", blake::Blake256, fixed_test);
