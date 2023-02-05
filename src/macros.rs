//#[macro_use]
/// macro that creates new Flag
/// for example: flag_new![3, 7] makes a flag that has those flags enabled (136)
#[macro_export]
macro_rules! flag_new {
    ( $($f:expr),* ) => {
        {
            let mut tflag = crate::Flag::new();
            $(
                tflag.set_flag($f, true);
            )*
            tflag
        }
    }
}
