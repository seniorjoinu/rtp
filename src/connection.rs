use crate::utils::Md4Hash;

const DEFAULT_MTU: u16 = 1200;

pub struct Connection {
    mtu: u16,
    interested_in: Vec<Md4Hash>,
}

impl Connection {
    fn new() -> Self {
        Connection {
            mtu: DEFAULT_MTU,
            interested_in: Vec::new(),
        }
    }

    fn add_interested_in(&mut self, thread_id: Md4Hash) {
        self.interested_in.push(thread_id)
    }

    fn is_interested_in(&self, thread_id: &Md4Hash) -> bool {
        self.interested_in.contains(thread_id)
    }

    fn remove_interested_in(&mut self, thread_id: &Md4Hash) {
        if let Some(index) = self.interested_in.iter().position(|it| *thread_id == *it) {
            self.interested_in.remove(index);
        }
    }
}
