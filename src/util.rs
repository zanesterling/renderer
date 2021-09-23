#[macro_export]
macro_rules! once {
    ($run_once:stmt) => {
        use std::sync::{Once};
        static SYNC_OBJ: Once = Once::new();
        SYNC_OBJ.call_once(|| {
            $run_once
        });
    }
}