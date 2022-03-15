fn main() {
}

#[async_trait::async_trait]
pub trait AnotherGroovyTrait{
    async fn groovy(&self) -> String {
        String::from("Groovy baby.")
    }
}

pub struct AustinPowers;
#[mockall::automock]
#[async_trait::async_trait]
pub trait GroovyTrait {
    async fn mojo(mini_me: &(dyn AnotherGroovyTrait + Send + Sync)) -> String;
}

#[async_trait::async_trait]
impl GroovyTrait for AustinPowers {
    async fn mojo(mini_me: &(dyn AnotherGroovyTrait + Send + Sync)) -> String {
        mini_me.groovy().await
    }
}