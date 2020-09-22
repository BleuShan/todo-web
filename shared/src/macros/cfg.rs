macro_rules! cfg_tracing_appender {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tracing-appender")]
            #[cfg_attr(docsrs, doc(cfg(feature = "tracing-appender")))]
            $item
        )*
    }
}

macro_rules! cfg_not_tracing_appender {
    ($($item:item)*) => {
        $(
            #[cfg(not(feature = "tracing-appender"))]
            #[cfg_attr(docsrs, doc(cfg(not(feature = "tracing-appender"))))]
            $item
        )*
    }
}
macro_rules! cfg_native {
       ($($item:item)*) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
            $item
        )*
    }
}

macro_rules! cfg_wasm {
       ($($item:item)*) => {
        $(
            #[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
            #[cfg_attr(docsrs, doc(cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))))]
            $item
        )*
    }
}
