use crate::core::{ Lark, LarkInner};

pub struct {{ serviceUpperCamelCase  }}Service<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> {{ serviceUpperCamelCase  }}Service<'_, Store, Client> {
    pub fn mock(&self) -> {{ serviceUpperCamelCase  }}ServiceMocker<Store, Client> {
        {{ serviceUpperCamelCase  }}ServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct {{ serviceUpperCamelCase  }}ServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn {{ service_snake_case }}(&self) -> {{ serviceUpperCamelCase  }}Service<Store, Client> {
        {{ serviceUpperCamelCase  }}Service { cli: &self.inner }
    }
}
