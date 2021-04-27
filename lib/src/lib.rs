pub mod mount;
pub mod pair;
pub mod unmount;
pub mod unpair;

pub use mount::mount_device;
pub use pair::pair_device;
pub use unmount::unmount_device;
pub use unpair::unpair_device;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
