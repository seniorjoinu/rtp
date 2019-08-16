use std::rc::Rc;

use crate::utils::Md4Hash;

const DEFAULT_MTU: u16 = 1200;

pub struct Connection {
    mtu: u16,
    interested_in: Vec<Rc<Md4Hash>>,
}

// TODO: very expensive implementation =C
impl Connection {
    fn new() -> Self {
        Connection {
            mtu: DEFAULT_MTU,
            interested_in: Vec::new(),
        }
    }

    fn add_interested_in(&mut self, message_id: Rc<Md4Hash>) {
        self.interested_in.push(message_id)
    }

    fn is_interested_in(&self, message_id: &Rc<Md4Hash>) -> bool {
        self.interested_in.contains(message_id)
    }

    fn remove_interested_in(&mut self, message_id: &Rc<Md4Hash>) {
        if let Some(index) = self.interested_in.iter().position(|it| *message_id == *it) {
            self.interested_in.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::connection::Connection;

    fn it_works() {
        let c = Connection::new();
        c.add_interested_in()
    }
}
