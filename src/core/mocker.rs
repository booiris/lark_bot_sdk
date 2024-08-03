use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Mutex, OnceLock},
};

use typemap_rev::{TypeMap, TypeMapKey};
pub struct MockerBuilder<Marker, Req, Resp, F> {
    instance_id: usize,
    mocker_data: MockerData<Req, F>,
    _marker: PhantomData<MockerMarker<Marker, Req, Resp, F>>,
}

pub struct Mocker<Marker: 'static, Req: 'static, Resp: 'static, F: Send + 'static> {
    instance_id: usize,
    _marker: PhantomData<MockerMarker<Marker, Req, Resp, F>>,
}

pub(crate) struct MockerData<Req, F> {
    times: Option<u8>,
    mock_times: u8,
    #[allow(clippy::type_complexity)]
    when: Option<Box<dyn Fn(&Req) -> bool + Send>>,
    f: F,
}

impl<Req, F> MockerData<Req, F> {
    pub(self) fn new(f: F) -> Self {
        Self {
            times: None,
            mock_times: 0,
            when: None,
            f,
        }
    }
}

struct MockerMarker<Marker, Req, Resp, F> {
    _req: PhantomData<Req>,
    _resp: PhantomData<Resp>,
    _marker: PhantomData<Marker>,
    _f: PhantomData<F>,
}
impl<Marker: 'static, Req: 'static, Resp: 'static, F: 'static + Send> TypeMapKey
    for MockerMarker<Marker, Req, Resp, F>
{
    type Value = Mutex<HashMap<usize, MockerData<Req, F>>>;
}

static MOCKER: OnceLock<Mutex<TypeMap>> = OnceLock::new();

fn mocker<'c, Marker: 'static, Req: 'static, Resp: 'static, F: 'static + Send>(
) -> &'c Mutex<TypeMap> {
    let mocker = MOCKER.get_or_init(|| Mutex::new(TypeMap::new()));
    mocker
        .lock()
        .unwrap()
        .entry::<MockerMarker<Marker, Req, Resp, F>>()
        .or_insert_with(|| Mutex::new(HashMap::new()));
    mocker
}

impl<Marker: 'static, Req: 'static, Resp: 'static, F: 'static + Send + Clone>
    MockerBuilder<Marker, Req, Resp, F>
{
    pub fn times<T: Into<u8>>(mut self, times: T) -> MockerBuilder<Marker, Req, Resp, F> {
        self.mocker_data.times = Some(times.into());
        self
    }

    pub fn when<When: Fn(&Req) -> bool + Send + 'static>(
        mut self,
        when: When,
    ) -> MockerBuilder<Marker, Req, Resp, F> {
        self.mocker_data.when = Some(Box::new(when));
        self
    }

    pub(crate) fn new(instance_id: usize, f: F) -> Self {
        Self {
            instance_id,
            mocker_data: MockerData::new(f),
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn build(self) -> Mocker<Marker, Req, Resp, F> {
        let binding = mocker::<Marker, Req, Resp, F>().lock().unwrap();
        let mut mocker = binding
            .get::<MockerMarker<Marker, Req, Resp, F>>()
            .unwrap()
            .lock()
            .unwrap();
        if mocker.contains_key(&self.instance_id) {
            panic!("mocker is set, please clear mocker first");
        }
        mocker.insert(self.instance_id, self.mocker_data);
        Mocker {
            instance_id: self.instance_id,
            _marker: PhantomData,
        }
    }
}

impl<Marker: 'static, Req: 'static, Resp: 'static, F: 'static + Send> Mocker<Marker, Req, Resp, F> {
    pub fn clear(self) {}
}

impl<Marker: 'static, Req: 'static, Resp: 'static, F: Send + 'static> Drop
    for Mocker<Marker, Req, Resp, F>
{
    fn drop(&mut self) {
        let binding = mocker::<Marker, Req, Resp, F>().lock().unwrap();
        let mut mocker = binding
            .get::<MockerMarker<Marker, Req, Resp, F>>()
            .unwrap()
            .lock()
            .unwrap();
        mocker.remove(&self.instance_id);
    }
}

pub(crate) fn do_mock<Marker: 'static, Req: 'static, Resp: 'static, F: 'static + Send + Clone>(
    instance_id: usize,
    req: &Req,
) -> Option<F> {
    let binding = mocker::<Marker, Req, Resp, F>().lock().unwrap();
    let mut mocker = binding
        .get::<MockerMarker<Marker, Req, Resp, F>>()
        .unwrap()
        .lock()
        .unwrap();
    if let Some(mocker) = mocker.get_mut(&instance_id) {
        if let Some(max_times) = mocker.times {
            if max_times <= mocker.mock_times {
                return None;
            }
        }
        if let Some(when) = &mocker.when {
            if !when(req) {
                return None;
            }
        }
        mocker.mock_times += 1;
        return Some(mocker.f.clone());
    }
    None
}
