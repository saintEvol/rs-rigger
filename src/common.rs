use bastion::children::Children;
use bastion::children_ref::ChildrenRef;
use bastion::prelude::{MessageHandler, SupervisorRef};
use bastion::supervisor::Supervisor;
use crate::{ChildrenType, gen_server, gen_supervisor, StartOpts};

// pub trait ChildrenPart
// {
//     fn new() -> Self;
//     // fn children_type() -> ChildrenType;
//     // fn options(&self) -> ChildrenOpts<T>;
// }

pub trait ChildrenSpec {
    fn wrap_gen_server() -> Children;
    fn wrap_gen_supervisor() -> Supervisor;
}

pub enum StartInitType {
    GenServer(Box<dyn FnOnce(Children) -> Children>),
    Supervisor(Box<dyn FnOnce(Supervisor) -> Supervisor>)
}

impl StartInitType {
    // pub fn GenServr
}

// impl<A, R> From<StartOpts<A, R>> for StartInitType
//     where
//         A: 'static + Send + Clone,
//         R: 'static + AsRef<str> + Send + Clone
// {
//     fn from(opts: StartOpts<A, R>) -> Self {
//         match opts.children_type {
//             ChildrenType::worker => {
//                 let init = gen_server::wrap_children(opts);
//                 StartInitType::GenServer(Box::new(init))
//             },
//             ChildrenType::supervisor => {
//                 let init = gen_supervisor::wrap_supervisor(opts);
//                 StartInitType::Supervisor(Box::new(init))
//             }
//         }
//     }
// }

pub enum StartResultType {
    ChildrenRef(ChildrenRef),
    SupervisorRef(SupervisorRef)
}


