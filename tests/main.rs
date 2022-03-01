extern crate deasync;

#[deasync::deasync]
async fn async_fn() {}

#[deasync::deasync]
pub async fn pub_async_fn() {}

#[deasync::deasync]
pub(crate) async fn pub_crate_async_fn() {}

#[deasync::deasync]
async unsafe fn async_unsafe_fn() {}

#[deasync::deasync]
pub trait PubTrait {
    async fn async_fn(&self);
}

#[deasync::deasync]
pub(crate) trait PubCrateTrait {
    async fn async_fn(&self);
}

struct Struct;

#[deasync::deasync]
impl PubTrait for Struct {
    async fn async_fn(&self) {
        async_fn().await
    }
}

#[deasync::deasync]
impl PubCrateTrait for Struct {
    async fn async_fn(&self) {
        async_fn().await
    }
}

#[test]
fn test_async_fn() {
    async_fn();
    pub_async_fn();
    pub_crate_async_fn();
    unsafe { async_unsafe_fn() };
    let s = Struct;
    PubTrait::async_fn(&s);
    PubCrateTrait::async_fn(&s);
}
