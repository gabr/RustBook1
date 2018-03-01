fn main() {
    println!("Hello, world!");
    println!("Test");

    //#[cfg(foo)]
    #[cfg(feature = "foo")]
    {
        println!("foo");
    }

    /*
        #[cfg(any(unix, windows))]
        #[cfg(all(unix, target_pointer_width = "32"))]
        #[cfg(not(foo))]

        These can nest arbitrarily:
        #[cfg(any(not(unix), all(target_os="macos", target_arch = "powerpc")))]
     */

}
