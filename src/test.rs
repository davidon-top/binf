use crate::{flag_new, Flag, flag_utils:: vec_to_flag};

#[test]
fn all() {
    let flag = flag_new![3, 7];
    assert_eq!(flag.get(), 136);
    assert_eq!(flag.get_flag(3), true);
    assert_eq!(flag.get_flag(4), false);
    assert_eq!(flag.get_all_flags(), vec![false, false, false, true, false, false, false, true]);
    assert_eq!(vec_to_flag(vec![false, false, false, true, false, false, false, true]), flag.get());
}